use std::env;
mod ayoze;
mod helpers;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //ayoze::spawn_players();
    let result = ayoze::write_app();
    if result.is_ok() {
        println!("Success!");
    } else {
        panic!("ERROR {:?}", result.err())
    }
    helpers::io::request_input();
}
