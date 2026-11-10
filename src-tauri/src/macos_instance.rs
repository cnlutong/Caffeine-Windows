use core_foundation::{base::TCFType, string::CFString};
use core_foundation_sys::{dictionary::CFDictionaryRef, notification_center::*};
use std::{
    ffi::c_void,
    fs::{self, File, OpenOptions},
    io,
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    ptr,
};
use tauri::AppHandle;

pub fn config_dir(identifier: &str) -> PathBuf {
    PathBuf::from(objc2_foundation::NSHomeDirectory().to_string())
        .join("Library/Application Support")
        .join(identifier)
}

pub struct Instance {
    _observer: ReopenObserver,
    // Keep the advisory lock for the lifetime of the app. The OS releases it on a crash.
    _lock: File,
}

impl Instance {
    pub fn acquire(app: &AppHandle, directory: &Path) -> io::Result<Option<Self>> {
        fs::create_dir_all(directory)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(directory.join("instance.lock"))?;

        // Register before locking so a concurrent launch can notify the winner during startup.
        let observer = ReopenObserver::new(app);
        loop {
            if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
                return Ok(Some(Self {
                    _observer: observer,
                    _lock: lock,
                }));
            }
            let error = io::Error::last_os_error();
            match error.kind() {
                io::ErrorKind::Interrupted => continue,
                io::ErrorKind::WouldBlock => {
                    observer.post();
                    return Ok(None);
                }
                _ => return Err(error),
            }
        }
    }
}

struct ReopenObserver {
    app: Box<AppHandle>,
    name: String,
}

impl ReopenObserver {
    fn new(app: &AppHandle) -> Self {
        let observer = Self {
            app: Box::new(app.clone()),
            name: format!("{}.show-settings", app.config().identifier),
        };
        let name = CFString::new(&observer.name);
        unsafe {
            CFNotificationCenterAddObserver(
                CFNotificationCenterGetDistributedCenter(),
                observer.context(),
                reopen,
                name.as_concrete_TypeRef(),
                ptr::null(),
                CFNotificationSuspensionBehaviorDeliverImmediately,
            );
        }
        observer
    }

    fn context(&self) -> *const c_void {
        ptr::from_ref(self.app.as_ref()).cast()
    }

    fn post(&self) {
        let name = CFString::new(&self.name);
        unsafe {
            CFNotificationCenterPostNotification(
                CFNotificationCenterGetDistributedCenter(),
                name.as_concrete_TypeRef(),
                ptr::null(),
                ptr::null(),
                1,
            );
        }
    }
}

impl Drop for ReopenObserver {
    fn drop(&mut self) {
        let name = CFString::new(&self.name);
        unsafe {
            CFNotificationCenterRemoveObserver(
                CFNotificationCenterGetDistributedCenter(),
                self.context(),
                name.as_concrete_TypeRef(),
                ptr::null(),
            );
        }
    }
}

extern "C" fn reopen(
    _center: CFNotificationCenterRef,
    observer: *mut c_void,
    _name: CFNotificationName,
    _object: *const c_void,
    _info: CFDictionaryRef,
) {
    // Distributed notifications arrive on the main thread. The observer owns this box
    // until it unregisters there. The notification only opens the settings window.
    let app = unsafe { &*observer.cast::<AppHandle>() };
    super::show_settings(app);
}
