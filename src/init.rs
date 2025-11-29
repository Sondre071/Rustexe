use windows_sys::Win32::System::Console::{
    CONSOLE_CURSOR_INFO, CONSOLE_SCREEN_BUFFER_INFO, ENABLE_EXTENDED_FLAGS, ENABLE_PROCESSED_INPUT,
    ENABLE_WINDOW_INPUT, GetConsoleMode, GetConsoleScreenBufferInfo, SetConsoleCursorInfo,
    SetConsoleMode,
};

use windows_sys::Win32::Foundation::HANDLE;

pub fn init_console(stdin: HANDLE, stdout: HANDLE) {
    unsafe {
        let mut old_mode = 0;

        let cursor_info = CONSOLE_CURSOR_INFO {
            dwSize: 1,
            bVisible: 0,
        };

        if GetConsoleMode(stdin, &mut old_mode) == 0 {
            panic!("Failed to get console mode");
        }

        let new_mode =
            old_mode | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS | ENABLE_PROCESSED_INPUT;

        if SetConsoleMode(stdin, new_mode) == 0 {
            panic!("Failed to set console mode");
        }

        SetConsoleCursorInfo(stdout, &cursor_info);
    }
}

pub fn get_terminal_width(stdout: HANDLE) -> i16 {
    unsafe {
        let mut csbi: CONSOLE_SCREEN_BUFFER_INFO = std::mem::zeroed();

        if GetConsoleScreenBufferInfo(stdout, &mut csbi) == 0 {
            panic!("Failed to get terminal width.")
        }

        let width = csbi.srWindow.Right - csbi.srWindow.Left + 1;

        width
    }
}
