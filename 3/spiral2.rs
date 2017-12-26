use std::collections::HashMap;
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

struct Spiral {
    map: HashMap<Position, i32>,
    direction: Direction,
    position: Position,
}

impl Spiral {
    fn new() -> Spiral {
        let origin = Position{ x: 0, y: 0 };
        let mut spiral = Spiral {
            map: HashMap::new(),
            direction: Direction::Down,
            position: origin,
        };
        spiral.map.insert(origin, 1);
        spiral
    }

    fn next_over(&mut self, limit: i32) -> i32 {
        loop {
            let curved_direction = self.next_direction();
            let curved_position = self.translated_position(curved_direction);
            if !self.map.contains_key(&curved_position) {
                // change direction
                self.direction = curved_direction;
                self.position = curved_position;
            } else {
                self.position = self.translated_position(self.direction);
            }
            let sum = self.sum_neighbors(self.position);
            self.map.insert(self.position, sum);
            if sum > limit as i32 {
                return sum;
            }
        }
    }

    fn sum_neighbors(&self, position: Position) -> i32 {
        let Position{x, y} = position;
        self.get(x+1, y)   + self.get(x-1, y)
            + self.get(x,   y+1) + self.get(x,   y-1)
            + self.get(x+1, y+1) + self.get(x-1, y-1)
            + self.get(x+1, y-1) + self.get(x-1, y+1)
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        self.map.get(&Position {x, y}).unwrap_or(&0).clone()
    }

    fn translated_position(&self, direction: Direction) -> Position {
        match direction {
            Direction::Down => {
                Position {
                    x: self.position.x,
                    y: self.position.y+1,
                }
            }
            Direction::Right => {
                Position {
                    x: self.position.x+1,
                    y: self.position.y,
                }
            }
            Direction::Up => {
                Position {
                    x: self.position.x,
                    y: self.position.y-1,
                }
            }
            Direction::Left => {
                Position {
                    x: self.position.x-1,
                    y: self.position.y,
                }
            }
        }
    }

    fn next_direction(&self) -> Direction {
        match self.direction {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }
}

fn get_argument() -> i32 {
    use std::io::Read;

    let mut line = String::new();
    io::stdin().read_to_string(&mut line).expect("Expected input line");
    return line.trim().parse::<i32>().unwrap();
}

fn main() {
    let limit = get_argument();
    let mut spiral = Spiral::new();
    let sum = spiral.next_over(limit);
    println!("{}", sum);
}
