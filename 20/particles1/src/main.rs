extern crate regex;

use std::cmp::Ordering;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
struct Vector(i32, i32, i32);

impl Vector {
    fn manhattan(&self) -> i32 {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

fn manhattan_order(a: &Vector, b: &Vector) -> Ordering {
    a.manhattan().cmp(&b.manhattan())
}

#[derive(Debug)]
struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
}

impl Particle {
    fn new() -> Particle {
        Particle {
            position: Vector(0, 0, 0),
            velocity: Vector(0, 0, 0),
            acceleration: Vector(0, 0, 0),
        }
    }
}

fn load_particles(reader: &mut BufRead) -> Vec<Particle> {
    let mut particles = Vec::new();
    let re = Regex::new(r"([pva])=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>").expect("bad regex");

    for line in reader.lines() {
        let line = line.expect("io error");
        let mut particle = Particle::new();

        for caps in re.captures_iter(&line) {
            set_particle_field(&mut particle, &caps[1], &caps[2], &caps[3], &caps[4]);
        }

        particles.push(particle);
    }

    particles
}

fn get_slowest_particle(particles: &Vec<Particle>) -> usize {
    fn ord(a: &Particle, b: &Particle) -> Ordering {
        manhattan_order(&a.acceleration, &b.acceleration)
            .then(manhattan_order(&a.velocity, &b.velocity))
            .then(manhattan_order(&a.position, &b.position))

    }

    particles.iter()
        .enumerate()
        .min_by(|&(_, particle_a), &(_, particle_b)| ord(particle_a, particle_b))
        .map(|(index, _)| index).expect("no particle found")
}

fn set_particle_field(particle: &mut Particle,
                      field_name: &str,
                      field_x: &str,
                      field_y: &str,
                      field_z: &str) {
    let value = Vector(field_x.parse::<i32>().expect("invalid x value"),
                       field_y.parse::<i32>().expect("invalid y value"),
                       field_z.parse::<i32>().expect("invalid z value"));

    match field_name {
        "a" => {
            particle.acceleration = value;
        }
        "p" => {
            particle.position = value;
        }
        "v" => {
            particle.velocity = value;
        }
        _ => {
            panic!("invalid field value {}", field_name);
        }
    }
}

fn main() {
    let stdin = std::io::stdin();

    let particles = load_particles(&mut stdin.lock());

    println!("{}", get_slowest_particle(&particles));
}
