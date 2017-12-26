extern crate regex;

use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Add;
use regex::Regex;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Vector(i64, i64, i64);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

#[derive(Clone, Copy, Debug)]
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

    fn update(&mut self) {
        self.velocity = self.velocity + self.acceleration;
        self.position = self.position + self.velocity;
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

fn get_remaining_particles(particles: &mut Vec<Particle>) -> usize {
    let mut idle_iters = 0;
    let mut prev_len = particles.len();

    loop {
        remove_colliding_particles(particles);

        let new_len = particles.len();
        if new_len < prev_len {
            prev_len = new_len;
            idle_iters = 0;
        }

        for particle in particles.iter_mut() {
            particle.update();
        }

        if prev_len == 0 {
            break;
        }

        idle_iters += 1;
        if idle_iters == 1000 {
            break;
        }
    }

    particles.len()
}

fn remove_colliding_particles(particles: &mut Vec<Particle>) {
    let mut map: HashMap<Vector, usize> = HashMap::new();

    for particle in particles.iter() {
        *map.entry(particle.position).or_insert(0) += 1;
    }

    particles.retain(|particle| *map.get(&particle.position).unwrap() <= 1);
}


fn set_particle_field(particle: &mut Particle,
                      field_name: &str,
                      field_x: &str,
                      field_y: &str,
                      field_z: &str) {
    let value = Vector(field_x.parse::<i64>().expect("invalid x value"),
                       field_y.parse::<i64>().expect("invalid y value"),
                       field_z.parse::<i64>().expect("invalid z value"));

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

    let mut particles = load_particles(&mut stdin.lock());

    println!("{}", get_remaining_particles(&mut particles));
}
