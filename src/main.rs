use windows_sys::Win32::System::Console::{
    ENABLE_EXTENDED_FLAGS, ENABLE_PROCESSED_INPUT, ENABLE_WINDOW_INPUT, GetConsoleMode,
    GetStdHandle, INPUT_RECORD, KEY_EVENT, ReadConsoleInputW, STD_INPUT_HANDLE, SetConsoleMode,
};

use windows_sys::Win32::Foundation::HANDLE;

fn main() {
    println!("Rustexe. Press q to quit.");

    unsafe {
        let stdin: HANDLE = GetStdHandle(STD_INPUT_HANDLE);

        let mut old_mode: u32 = 0;
        if GetConsoleMode(stdin, &mut old_mode) == 0 {
            println!("Failed to get console mode.");
            return;
        }

        let new_mode =
            old_mode | ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS | ENABLE_PROCESSED_INPUT;

        if SetConsoleMode(stdin, new_mode) == 0 {
            println!("Failed to set console mode.");
            return;
        }

        let mut records: [INPUT_RECORD; 16] = std::mem::zeroed();
        let mut events_read = 0;

        loop {
            let ok = ReadConsoleInputW(
                stdin,
                records.as_mut_ptr(),
                records.len() as u32,
                &mut events_read,
            );

            if ok == 0 {
                println!("ReadConsoleInputW failed.");
                break;
            }

            // 5. Process each event
            for rec in &records[..events_read as usize] {
                if rec.EventType as u32 == KEY_EVENT {
                    // INPUT_RECORD has a union named Event with a field KeyEvent
                    let event = rec.Event.KeyEvent;

                    let is_key_down = event.bKeyDown != 0;
                    let ch = { event.uChar.UnicodeChar }; // uChar is also a union
                    let vk = event.wVirtualKeyCode;

                    if is_key_down {
                        // Show raw data
                        println!(
                            "Key event: down={} vk=0x{:02X} unicode={:?}",
                            is_key_down,
                            vk,
                            char::from_u32(ch as u32)
                        );

                        // Quit on 'q'
                        if ch == 'q' as u16 {
                            println!("Quitting.");
                            // Restore console mode
                            SetConsoleMode(stdin, old_mode);
                            return;
                        }
                    }
                }
            }
        }
    }
}
