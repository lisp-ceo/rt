use core::Tuple;
use core::normalize;

fn main() {
    let p = Projectile{
        position: Tuple::point(0.0,1.0,0.0),
        velocity: normalize(Tuple::vector(1.0,1.0,0.0))
    };
    let e = Environment{
        gravity: Tuple::vector(0.0,-0.1,0.0),
        wind: Tuple::vector(-0.01,0.0,0.0)
    };
    let mut optional = Some(p);
    loop {
        match optional {
            Some(p) => {
                summarize(&p);
                if p.position.y <= 0.0 {
                    optional = None;
                } else {
                    optional = Some(tick(&e, &p))
                }
            },
            _ => {
                println!("Our projectile has stopped!");
                break;
            }
        }
    }
}

fn summarize(i: &Projectile) {
    println!("{:#?}", i)
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
fn tick(e: &Environment, p: &Projectile) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + e.gravity + e.wind;
    Projectile{position, velocity}
}
