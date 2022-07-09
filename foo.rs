fn main() {
    let args: Vec<String> = env::args().collect();
    ayoze::spawn_players();
    helpers::io::request_input();
}
