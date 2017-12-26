use std::io::Read;

fn get_max_distance(directions: &Vec<&str>) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut max_distance: i32 = 0;

    for dir in directions {
        match *dir {
            "n" => {
                y += 1;
                z -= 1;
            }
            "ne" => {
                x += 1;
                z -= 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "s" => {
                y -= 1;
                z += 1;
            }
            "sw" => {
                x -= 1;
                z += 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            _ => {
                panic!("invalid direction '{}'", dir);
            }
        }

        max_distance = max_distance.max((x.abs() + y.abs() + z.abs()) / 2)
    }

    max_distance
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("missing input");

    let directions: Vec<&str> = input.trim().split(',').collect::<Vec<_>>();
    println!("{}", get_max_distance(&directions));
}
