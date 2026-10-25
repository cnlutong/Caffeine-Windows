#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::PowerRequest;
#[cfg(target_os = "windows")]
pub use windows::PowerRequest;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
compile_error!("Caffeine 2.0 supports Windows x86_64 and macOS aarch64 only.");

#[cfg(any(
    all(target_os = "windows", not(target_arch = "x86_64")),
    all(target_os = "macos", not(target_arch = "aarch64"))
))]
compile_error!("Build Windows for x86_64 or macOS for aarch64.");
