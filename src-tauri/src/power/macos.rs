use crate::model::{Mode, PowerBackend};
use core_foundation::{
    base::TCFType,
    string::{CFString, CFStringRef},
};

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOPMAssertionCreateWithName(
        kind: CFStringRef,
        level: u32,
        name: CFStringRef,
        id: *mut u32,
    ) -> i32;
    fn IOPMAssertionRelease(id: u32) -> i32;
}

#[derive(Default)]
pub struct PowerRequest {
    assertion: Option<u32>,
}

impl PowerBackend for PowerRequest {
    fn apply(&mut self, mode: Mode) -> Result<(), String> {
        let next = match mode {
            Mode::Off => None,
            Mode::System | Mode::Display => {
                let kind = CFString::new(if mode == Mode::System {
                    "PreventUserIdleSystemSleep"
                } else {
                    "PreventUserIdleDisplaySleep"
                });
                let reason = CFString::new("Caffeine: user enabled wake prevention");
                let mut id = 0;
                // Both strings stay alive through the synchronous call. IOKit copies the properties.
                let result = unsafe {
                    IOPMAssertionCreateWithName(
                        kind.as_concrete_TypeRef(),
                        255,
                        reason.as_concrete_TypeRef(),
                        &mut id,
                    )
                };
                if result != 0 {
                    return Err(format!("IOPMAssertionCreateWithName: {result:#x}"));
                }
                Some(id)
            }
        };
        if let Some(old) = self.assertion {
            let result = unsafe { IOPMAssertionRelease(old) };
            if result != 0 {
                if let Some(id) = next {
                    unsafe {
                        IOPMAssertionRelease(id);
                    }
                }
                return Err(format!("IOPMAssertionRelease: {result:#x}"));
            }
        }
        self.assertion = next;
        Ok(())
    }
}

impl Drop for PowerRequest {
    fn drop(&mut self) {
        if let Some(id) = self.assertion.take() {
            unsafe {
                IOPMAssertionRelease(id);
            }
        }
    }
}
