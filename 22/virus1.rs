use std::collections::HashSet;
use std::io::Read;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32);

type Grid = HashSet<Coord>;

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
                b'.' => {},
                b'#' => {
                    grid.insert(Coord(x as i32 - column_count / 2, y as i32 - row_count / 2));
                },
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
        if grid.contains(&pos) {
            dir = turn_right(dir);
            grid.remove(&pos);
        } else {
            dir = turn_left(dir);
            grid.insert(pos);
            num_infections += 1;
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

    println!("{}", count_infections(&mut grid, 10000));
}
