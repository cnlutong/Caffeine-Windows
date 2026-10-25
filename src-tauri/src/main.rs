#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if std::env::args().any(|arg| arg == "--self-test") {
        if let Err(error) = caffeine_lib::self_test() {
            eprintln!("Caffeine self-test failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    caffeine_lib::run();
}
