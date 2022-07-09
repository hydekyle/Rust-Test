pub fn spawn_players() {
    println!("Spawning players!");
    let mut player_list = Vec::new();
    let ayoze = Player {
        name: String::from("Ayoze"),
        status: Status::Awaken,
        position: None,
    };
    player_list.push(ayoze);
    let manuel = Player {
        name: String::from("Manuel"),
        status: Status::Awaken,
        position: Some(Point(1, 2, 3)),
    };
    player_list.push(manuel);
    for mut player in player_list {
        player.say_my_name();
        player.spawn();
    }
}

#[derive(Debug)]
pub struct Point(pub i32, pub i32, pub i32);

pub enum Status {
    Awaken,
    Slept,
}

pub struct Player {
    pub name: String,
    pub status: Status,
    pub position: Option<Point>,
}

impl Player {
    pub fn spawn(&mut self) {
        if self.position.is_none() {
            self.position = Some(Point(0, 0, 0));
        }
        self.say_my_name();
    }
    fn say_my_name(&self) {
        println!("Soy {} y estoy en {:?}", self.name, self.position);
    }
}

pub fn write_app() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    let text = "fn main() {
        let args: Vec<String> = env::args().collect();
        ayoze::spawn_players();
        helpers::io::request_input();
    }";
    let mut file = File::create("foo.rs")?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
