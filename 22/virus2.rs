use std::collections::HashMap;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32);

enum Status {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type Grid = HashMap<Coord, Status>;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn load_grid(input: &str) -> Grid {
    let mut grid = Grid::new();

    let rows = input.split_whitespace().collect::<Vec<&str>>();
    let row_count = rows.len() as i32;

    for (y, row) in rows.iter().enumerate() {
        let column_count = row.len() as i32;

        for (x, ch) in row.bytes().enumerate() {
            match ch {
                b'.' => {}
                b'#' => {
                    grid.insert(Coord(x as i32 - column_count / 2, y as i32 - row_count / 2),
                                Status::Infected);
                }
                _ => {
                    panic!("invalid input ({})", ch);
                }
            };
        }
    }

    grid
}

fn count_infections(grid: &mut Grid, num_steps: usize) -> usize {
    let mut num_infections = 0;
    let mut dir = Direction::Up;
    let mut pos = Coord(0, 0);

    for _ in 0..num_steps {
        match *grid.get(&pos).unwrap_or(&Status::Clean) {
            Status::Clean => {
                dir = turn_left(dir);
                grid.insert(pos, Status::Weakened);
            },
            Status::Weakened => {
                grid.insert(pos, Status::Infected);
                num_infections += 1;
            }
            Status::Infected => {
                dir = turn_right(dir);
                grid.insert(pos, Status::Flagged);
            }
            Status::Flagged => {
                dir = turn_left(turn_left(dir));
                grid.insert(pos, Status::Clean);
            }
        }

        pos = advance(pos, dir);
    }

    num_infections
}

fn turn_left(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down,
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn advance(pos: Coord, dir: Direction) -> Coord {
    match dir {
        Direction::Up => Coord(pos.0, pos.1 - 1),
        Direction::Right => Coord(pos.0 + 1, pos.1),
        Direction::Down => Coord(pos.0, pos.1 + 1),
        Direction::Left => Coord(pos.0 - 1, pos.1),
    }
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_to_string(&mut input).expect("io error");
    let mut grid = load_grid(&input);

    println!("{}", count_infections(&mut grid, 10000000));
}
