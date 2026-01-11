enum Direction {
    North,
    South,
    East,
    West
}

enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}


fn process(msg: Message) {
    match msg {
        Message::Quit => println!("Quit Requestd"),
        Message::Write(text) => println!("Message: {}", text),
        Message::Move { x, y } => println!("Moving: x: {}, y: {}", x,y),
    }
}

fn move_player(dir: Direction) {
 // Implementation Later
    describe_direction(dir);
}

fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Goung Up"),
        Direction::South => println!("Gowing Down"),
        Direction::East => println!("Going Right"),
        Direction::West => println!("Going Left")
    }
}

fn direction_value(dir: Direction) -> i32 {
    match dir {
        Direction::North => 1,
        Direction::South => -1,
        Direction::East => 2,
        Direction::West => -2,
    }
}

fn main() {
    let d = Direction::North;
    move_player(d);
}