use simulator_lib::{ObjectType, Planet, Asteroid, start_server};
use simulator_lib::directions::{
    coordinate::Coordinate,
    direction::Vector
};
fn main() {
    let mut objects = vec![
        ObjectType::Planet(Planet {
            coordinate: Coordinate::new(500, 500),
            weight: 50,
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(250, 250),
            velocity: Vector {x: 30, y: -10},
        }),
        ObjectType::Asteroid(Asteroid {
            coordinate: Coordinate::new(750, 750),
            velocity: Vector {x: -30, y: 10},
        }),
    ];

    start_server("0.0.0.0:16991", objects, 70);



}
