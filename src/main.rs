mod init;
mod input;
mod menu;

fn main() {
    println!("Rustexe. Press q to quit.");

    let options = ["sahfsa", "jasdjfafdjido", "iasjdfaoijds", "adsoijf"];

    menu::run(&options);
}
