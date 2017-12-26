#![feature(slice_patterns)]
#![feature(slice_rotate)]

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn parse_moves(input: &str) -> Vec<Move> {
    let mut moves = Vec::new();

    for move_str in input.split(',').map(|token| token.trim()) {
        let (kind, positions) = move_str.split_at(1);
        moves.push(match kind {
            "s" => Move::Spin(positions.parse::<usize>().expect("invalid spin")),
            "x" => {
                let mut programs = positions.split('/')
                    .map(|token| token.parse::<usize>().expect("invalid program"));
                Move::Exchange(programs.next().expect("missing first program"),
                               programs.next().expect("missing second program"))
            }
            "p" => {
                let position_chars = positions.chars().collect::<Vec<_>>();
                if let [prog_a, '/', prog_b] = position_chars[..] {
                    Move::Partner(prog_a, prog_b)
                } else {
                    panic!("invalid program specification");
                }
            }
            _ => {
                panic!("Invalid move");
            }
        });
    }

    moves
}

fn execute_moves(moves: &Vec<Move>) -> String {
    let mut programs = (('a' as u8)..('q' as u8)).map(|ch| ch as char).collect::<Vec<char>>();

    for m in moves.iter() {
        match m {
            &Move::Spin(count) => {
                let count = programs.len() - count;
                programs.rotate(count);
            }
            &Move::Exchange(a, b) => {
                programs.swap(a, b);
            }
            &Move::Partner(a, b) => {
                for p in programs.iter_mut() {
                    if *p == a {
                        *p = b;
                    } else if *p == b {
                        *p = a;
                    }
                }
            }
        }
    }

    programs.iter().collect::<String>()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    let moves = parse_moves(&input);
    println!("{}", execute_moves(&moves));
}
