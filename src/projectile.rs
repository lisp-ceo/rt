fn main() {
    let p = Projectile{
        position: Tuple::point(0.0,1.0,0.0),
        velocity: normalize(vector(1.0,1.0,0.0))
    }
    let e = Environment{
        gravity: vector(0.0,-0.1,0.0),
        wind: Tuple::vector(-0.01,0.0,0.0)
    }
    println!(e);
    println!(p);
    let nu = tick(e, p);
    println!(nu);
}

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple
}
#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple
}

// returns a new projectile, representing the given projectile after one unit of time has passed
fn tick(e: Environment, p: Projectile) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + e.gravity + e.wind;
    Projectile{position, velocity}
}
