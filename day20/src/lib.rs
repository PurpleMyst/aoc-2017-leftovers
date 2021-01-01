use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pos: [i64; 3],
    vel: [i64; 3],
    accel: [i64; 3],
}

fn parse_triplet(var: &str) -> [i64; 3] {
    let var = &var["x=<".len()..var.len() - ">".len()];
    let mut parts = var.splitn(3, ',').map(|n| n.parse().unwrap());
    [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}

impl Particle {
    fn from_input(line: &str) -> Self {
        let mut parts = line.splitn(3, ", ");

        Self {
            pos: parse_triplet(parts.next().unwrap()),
            vel: parse_triplet(parts.next().unwrap()),
            accel: parse_triplet(parts.next().unwrap()),
        }
    }

    fn pos_at(&self, t: i64) -> [i64; 3] {
        let [x, y, z] = self.pos;
        let [vx, vy, vz] = self.vel;
        let [ax, ay, az] = self.accel;

        [
            ax * t * t + vx * t + x,
            ay * t * t + vy * t + y,
            az * t * t + vz * t + z,
        ]
    }

    fn dist_at(&self, t: i64) -> i64 {
        let [x, y, z] = self.pos_at(t);
        x.abs() + y.abs() + z.abs()
    }

    fn tick(&mut self) {
        let [ax, ay, az] = self.accel;

        let [vx, vy, vz] = &mut self.vel;
        *vx += ax;
        *vy += ay;
        *vz += az;

        let [x, y, z] = &mut self.pos;
        *x += *vx;
        *y += *vy;
        *z += *vz;
    }
}

#[inline]
pub fn solve_part1(particles: &[Particle]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.dist_at(100_000))
        .unwrap()
        .0
}

#[inline]
pub fn load_input() -> Vec<Particle> {
    include_str!("input.txt")
        .lines()
        .map(Particle::from_input)
        .collect::<Vec<_>>()
}

#[inline]
pub fn solve_part2(particles: &mut Vec<Particle>) -> usize {
    let mut positions = HashMap::new();
    let mut prev_len = particles.len();
    let mut steady = 0;

    loop {
        positions.clear();

        for particle in particles.iter_mut() {
            particle.tick();
            *positions.entry(particle.pos).or_default() += 1;
        }

        particles.retain(|particle| positions.get(&particle.pos) == Some(&1));

        if prev_len == particles.len() {
            steady += 1;

            if steady == 10 {
                break particles.len();
            }
        } else {
            prev_len = particles.len();
            steady = 0;
        }
    }
}

#[inline]
pub fn solve() -> (usize, usize) {
    let mut particles = load_input();
    (solve_part1(&particles), solve_part2(&mut particles))
}
