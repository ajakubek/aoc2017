use std::io::BufRead;
use std::ops;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Coord(i32, i32);

impl ops::Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Coord {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Coord {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

struct Maze {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Maze {
    fn load(reader: &mut BufRead) -> Maze {
        let mut maze = Maze {
            map: Vec::new(),
            width: 0,
            height: 0,
        };

        let mut width: Option<usize> = None;

        for line in reader.lines() {
            let chars = line.expect("io error").chars().collect::<Vec<char>>();
            if chars.len() > 0 {

                if let Some(previous_width) = width {
                    if chars.len() != previous_width {
                        panic!("inconsistent line width {}", chars.len());
                    }
                } else {
                    width = Some(chars.len());
                }

                maze.map.push(chars);
                maze.height += 1;
            }
        }

        maze.width = width.expect("empty maze");
        maze
    }

    fn collect_chars(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        let mut previous_pos = self.find_start();
        let mut pos = Coord(previous_pos.0, previous_pos.1 + 1);

        loop {
            match self.get(&pos) {
                ch @ 'A'...'Z' => {
                    chars.push(ch);
                    self.advance_position(&mut pos, &mut previous_pos);
                }
                '|' | '-' => {
                    self.advance_position(&mut pos, &mut previous_pos);
                }
                '+' => {
                    if let Some(next_pos) = self.get_changed_direction(pos, previous_pos) {
                        previous_pos = pos;
                        pos = next_pos;
                    } else {
                        break;
                    }
                }
                ' ' => {
                    break;
                }
                _ => {
                    panic!("Visited invalid char at {:?}", pos);
                }
            }

        }

        chars.iter().collect()
    }

    fn find_start(&self) -> Coord {
        Coord(self.map[0].iter().position(|x| *x == '|').expect("missing start") as i32,
              0)
    }

    fn advance_position(&self, current_pos: &mut Coord, previous_pos: &mut Coord) {
        let delta = *current_pos - *previous_pos;
        *previous_pos = *current_pos;
        *current_pos = *current_pos + delta
    }

    fn get_changed_direction(&self, current_pos: Coord, previous_pos: Coord) -> Option<Coord> {
        let delta = current_pos - previous_pos;
        let advanced_pos = current_pos + delta;

        let offsets = [Coord(-1, 0), Coord(0, 1), Coord(1, 0), Coord(0, -1)];
        for offset in offsets.iter() {
            let next_pos = current_pos + *offset;
            let next_char = self.get(&next_pos);
            if next_char != ' ' && next_pos != previous_pos && next_pos != advanced_pos {
                return Some(next_pos);
            }
        }

        None
    }

    fn get(&self, coord: &Coord) -> char {
        let &Coord(x, y) = coord;
        let x = x as usize;
        let y = y as usize;

        if y < self.map.len() {
            if x < self.map[y].len() {
                return self.map[y][x];
            }
        }

        ' '
    }
}

fn main() {
    let stdin = std::io::stdin();
    let maze = Maze::load(&mut stdin.lock());
    println!("{}", maze.collect_chars());
}
