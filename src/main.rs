mod init;
mod input;

fn main() {
    println!("Rustexe. Press q to quit.");

    let stdin = init::init_console();

    loop {
        let key = input::read_key_blocking(stdin);

        println!("Got: {}", key.ch.unwrap());

        if key.is_char('q') {
            break;
        }
    }
}
