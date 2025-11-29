use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};

use crate::{init, input};

pub fn run(options: &[&str]) {
    let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    init::init_console(stdin, stdout);
    let width = init::get_terminal_width(stdout) as usize;

    let l = options.len();
    let mut current_index = 0;

    render_menu(options, current_index, width);

    loop {
        let key = input::read_key_blocking(stdin);

        if let Some(ch) = key.ch {
            match ch {
                'j' => {
                    if current_index != l - 1 {
                        current_index += 1
                    }
                }
                'k' => {
                    if current_index != 0 {
                        current_index -= 1
                    }
                }
                'q' => {
                    println!("\x1b[0J");
                    break;
                }
                _ => continue,
            }
        }

        println!("\x1b[{}A", l + 1);

        render_menu(options, current_index, width);
    }
}

fn render_menu(options: &[&str], current: usize, width: usize) {
    let content_width = width.saturating_sub(3);

    for i in 0..options.len() {
        if i == current {
            println!(
                "\x1b[0;93m > {: <num$}\x1b[0m",
                options[i],
                num = content_width
            );
        } else {
            println!("   {: <num$}", options[i], num = content_width);
        }
    }
}
