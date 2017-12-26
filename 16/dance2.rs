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

fn apply_permutation_count(moves: &Vec<Move>, count: usize) -> Vec<char> {
    let cycle_count = find_cycle(moves);
    let remaining_count = count % cycle_count;
    let mut programs = generate_initial_programs();

    for _ in 0..remaining_count {
        apply_permutation(&mut programs, moves);
    }

    programs
}

fn find_cycle(moves: &Vec<Move>) -> usize {
    let initial = generate_initial_programs();
    let mut permuted = initial.to_vec();
    let mut counter = 0;

    loop {
        apply_permutation(&mut permuted, moves);
        counter += 1;
        if permuted == initial {
            return counter;
        }
    }
}

fn apply_permutation(programs: &mut Vec<char>, moves: &Vec<Move>) {
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
}

fn generate_initial_programs() -> Vec<char> {
    (b'a'..b'q').map(|ch| ch as char).collect()
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("io error");
    let moves = parse_moves(&input);
    let programs = apply_permutation_count(&moves, 1000000000);
    println!("{}", programs.iter().collect::<String>());
}
