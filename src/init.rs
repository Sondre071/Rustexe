use windows_sys::Win32::System::Console::{
    ENABLE_EXTENDED_FLAGS, ENABLE_PROCESSED_INPUT, ENABLE_WINDOW_INPUT, GetConsoleMode,
    GetStdHandle, STD_INPUT_HANDLE, SetConsoleMode,
};

use windows_sys::Win32::Foundation::HANDLE;

pub fn init_console() -> HANDLE {
    unsafe {
        let stdin = GetStdHandle(STD_INPUT_HANDLE);

        let mut old_mode = 0;
        if GetConsoleMode(stdin, &mut old_mode) == 0 {
            panic!("Failed to get console mode");
        }

        let new_mode =
            old_mode | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS | ENABLE_PROCESSED_INPUT;

        if SetConsoleMode(stdin, new_mode) == 0 {
            panic!("Failed to set console mode");
        }

        stdin
    }
}
