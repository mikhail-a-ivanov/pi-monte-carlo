use rand::prelude::*;

/// 2D Point struct
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

/// Generates a random 2D point
fn generate_random_point(rng: &mut ThreadRng) -> Point {
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    // Return point{ x, y }
    Point { x, y }
}

/// Computes a radius for a 2D point with x and y in [0, 1)
fn compute_radius(point: Point) -> f64 {
    let radius: f64 = point.x * point.x + point.y * point.y;
    // Return radius
    radius
}

/// Estimates pi using Monte Carlo algorithm
fn estimate_pi(n: i32) -> f64 {
    let mut rng = thread_rng();
    let mut counter: i32 = 0;
    for _ in 0..n {
        let point = generate_random_point(&mut rng);
        let radius: f64 = compute_radius(point);
        if radius < 1.0 {
            counter += 1;
        }
    }
    let pi: f64 = 4.0 * (counter as f64) / (n as f64);
    // Return estimate of pi
    pi
}

fn main() {
    const N: i32 = 1_000_000;
    let pi = estimate_pi(N);
    println!("Estimate of pi: {}", pi);
}
