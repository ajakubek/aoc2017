use std::collections::HashSet;
use std::io;

#[derive(Clone, Copy)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

fn spiral_coords(index: i32) -> Position {
    let mut map: HashSet<Position> = HashSet::new();
    let mut direction = Direction::Down;
    let mut position = Position { x: 0, y: 0 };

    map.insert(position);

    for _ in 1..index {
        let curved_direction = next_direction(direction);
        let curved_position = translate_position(position, curved_direction);
        if !map.contains(&curved_position) {
            // change direction
            direction = curved_direction;
            position = curved_position;
        } else {
            position = translate_position(position, direction);
        }
        map.insert(position);
    }

    position
}

fn translate_position(position: Position, direction: Direction) -> Position {
    match direction {
        Direction::Down => Position {x: position.x, y: position.y+1},
        Direction::Right => Position {x: position.x+1, y: position.y},
        Direction::Up => Position {x: position.x, y: position.y-1},
        Direction::Left => Position {x: position.x-1, y: position.y},
    }
}

fn next_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
    }
}

fn manhattan_distance(coords: Position) -> u32 {
    coords.x.abs() as u32 + coords.y.abs() as u32
}

fn get_argument() -> i32 {
    use std::io::Read;

    let mut line = String::new();
    io::stdin().read_to_string(&mut line).expect("Expected input line");
    return line.trim().parse::<i32>().unwrap();
}

fn main() {
    let index = get_argument();
    let coords = spiral_coords(index);
    println!("{}", manhattan_distance(coords));
}
