#[derive(Debug)]
pub struct Site {
    pub domain: String,
    pub name: String,
    pub nation: String,
    pub found: u32,
}
enum Direction {
    East,
    West,
    North,
    South,
}

fn enu_main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        Direction::West => todo!(),
    };
}
