use crate::model::{Mode, PowerBackend};
use windows_sys::Win32::{
    Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Power::{
            PowerClearRequest, PowerCreateRequest, PowerRequestDisplayRequired,
            PowerRequestSystemRequired, PowerSetRequest,
        },
        SystemServices::POWER_REQUEST_CONTEXT_VERSION,
        Threading::{POWER_REQUEST_CONTEXT_SIMPLE_STRING, REASON_CONTEXT, REASON_CONTEXT_0},
    },
};

struct Request {
    handle: HANDLE,
    system: bool,
    display: bool,
}
// The Windows power-request object is a process-owned kernel handle. Access is serialized by the app mutex.
unsafe impl Send for Request {}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe {
            if self.display {
                PowerClearRequest(self.handle, PowerRequestDisplayRequired);
            }
            if self.system {
                PowerClearRequest(self.handle, PowerRequestSystemRequired);
            }
            CloseHandle(self.handle);
        }
    }
}

#[derive(Default)]
pub struct PowerRequest {
    request: Option<Request>,
}

impl PowerBackend for PowerRequest {
    fn apply(&mut self, mode: Mode) -> Result<(), String> {
        if mode == Mode::Off {
            self.request = None;
            return Ok(());
        }
        let mut reason: Vec<u16> = "Caffeine: user enabled wake prevention\0"
            .encode_utf16()
            .collect();
        let context = REASON_CONTEXT {
            Version: POWER_REQUEST_CONTEXT_VERSION,
            Flags: POWER_REQUEST_CONTEXT_SIMPLE_STRING,
            Reason: REASON_CONTEXT_0 {
                SimpleReasonString: reason.as_mut_ptr(),
            },
        };
        // PowerCreateRequest copies the context; the UTF-16 string lives through the call.
        let handle = unsafe { PowerCreateRequest(&context) };
        if handle == INVALID_HANDLE_VALUE {
            return Err(format!("PowerCreateRequest: {}", unsafe { GetLastError() }));
        }
        let mut next = Request {
            handle,
            system: false,
            display: false,
        };
        if unsafe { PowerSetRequest(handle, PowerRequestSystemRequired) } == 0 {
            return Err(format!("PowerSetRequest(system): {}", unsafe {
                GetLastError()
            }));
        }
        next.system = true;
        if mode == Mode::Display {
            if unsafe { PowerSetRequest(handle, PowerRequestDisplayRequired) } == 0 {
                return Err(format!("PowerSetRequest(display): {}", unsafe {
                    GetLastError()
                }));
            }
            next.display = true;
        }
        // Activate the new request before dropping the old one. Failure leaves the old mode intact.
        self.request = Some(next);
        Ok(())
    }
}
