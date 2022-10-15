pub mod directions;

use crate::directions::{
    coordinate::Coordinate,
    direction::Vector
};

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Circle {
    cx: i32,
    cy: i32,
    r: i32,
    stroke: String,
    fill: String,
    #[serde(rename = "stroke-width")]
    stroke_width: i32,
}

#[derive(Clone)]
pub struct Planet {
    pub coordinate: Coordinate,
    pub weight: i32,
}

impl Planet {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn get_weight(&self) -> i32 {
        self.weight
    }
}

#[derive(Clone)]
pub struct Asteroid {
    pub coordinate: Coordinate,
    pub velocity: Vector,
}

impl Asteroid {
    fn get_location(&self) -> Coordinate {
        self.coordinate.clone()
    }

    fn get_velocity(&self) -> Vector {
        self.velocity.clone()
    }

    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 2,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

#[derive(Clone)]
pub enum GravityType {
    High,
    Low,
}

impl From<GravityType> for i32 {
    fn from(g_type: GravityType) -> Self {
        match g_type {
            GravityType::High => 2_i32,
            GravityType::Low => 1_i32,
        }
    }
}

pub struct CursedPlanet {
    previous_state: GravityType,
    weight: i32,
    coordinate: Coordinate
}
impl IntoCircle for CursedPlanet {
    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: 2,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
 }
pub trait Position {
    fn get_position(&self) -> Coordinate;
}

pub trait GravitySource: Position {
    fn get_weight(&self) -> i32;
}

pub trait IntoCircle {
    fn as_circle(&self) -> Circle;
}

trait CircularGravitySource: GravitySource + IntoCircle {

}

impl Position for Planet {
    fn get_position(&self) -> Coordinate {
        self.get_location()
    }
}

impl IntoCircle for Planet {
    fn as_circle(&self) -> Circle {
        Circle {
            cx: self.coordinate.x,
            cy: self.coordinate.y,
            r: self.weight,
            stroke: "green".to_string(),
            fill: "black".to_string(),
            stroke_width: 3,
        }
    }
}

impl CircularGravitySource for Planet {
}

impl GravitySource for Planet {
    fn get_weight(&self) -> i32 {
        self.get_weight()
    }
}

impl Position for Asteroid {
    fn get_position(&self) -> Coordinate {
        self.get_location()
    }
}

impl Position for CursedPlanet {
    fn get_position(&self) -> Coordinate {
        self.coordinate.clone()
    }
}

impl GravitySource for CursedPlanet {
    fn get_weight(&self) -> i32 {
        let ret_val = (self.previous_state.clone() as i32) * self.weight;
        // self.previous_state = match self.previous_state {
        //     GravityType::High => GravityType::Low,
        //     GravityType::Low => GravityType::High,
        // };
        ret_val
    }
}

#[derive(Clone)]
pub enum ObjectType {
    Planet(Planet),
    Asteroid(Asteroid),
}

impl ObjectType {
    fn get_circle(&self) -> Circle {
        match self {
            ObjectType::Planet(p) => p.as_circle(),
            ObjectType::Asteroid(a) => a.as_circle(),
        }
    }
}

fn get_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f64).sqrt() as i32
}

fn apply_physics(gravity_sources: Vec<Box<dyn CircularGravitySource>>, mut asteroids: Vec<Asteroid>, gravitational_constant: i32) -> (Vec<Box<dyn CircularGravitySource>>, Vec<Asteroid>) 
{    // Go through each pair of objects, and apply
    let gravity_source_tuples = gravity_sources.iter().map(|p|
            (p.get_position(), p.get_weight())).collect::<Vec<_>>();

    asteroids.iter_mut().for_each(|asteroid| {
        gravity_source_tuples.iter().for_each(|(planet_coord, planet_weight)| {
            let distance = get_distance(
                planet_coord.x, planet_coord.y,
                asteroid.coordinate.x, asteroid.coordinate.y
            );
            let distance = distance * distance;

            let force = Vector {
                x: (asteroid.coordinate.x - planet_coord.x) * planet_weight * gravitational_constant / distance,
                y: (asteroid.coordinate.y - planet_coord.y) * planet_weight * gravitational_constant / distance,
            };
            asteroid.velocity.x -= force.x;
            asteroid.velocity.y -= force.y;

            let vel = asteroid.velocity.clone();
        }) 
    });

    // Apply the new velocity to each object.
    asteroids.iter_mut().for_each(|asteroid| {
            asteroid.coordinate.x += asteroid.velocity.x;
            asteroid.coordinate.y += asteroid.velocity.y;
    });

    (gravity_sources, asteroids)
}

fn handle_connection(mut stream: TcpStream, mut objects: Vec<ObjectType>, gravitational_constant: i32) -> Vec<ObjectType> {
    let mut input_planets:Vec<Box<dyn CircularGravitySource>> = vec![];
    let mut input_asteroids = vec![];
    let mut planets:Vec<Box<dyn CircularGravitySource>> = vec![];
    let mut asteroids:Vec<Asteroid> = vec![];
    
    objects.iter().for_each(|object| 
        match object {
                ObjectType::Planet(planet) => input_planets.push(Box::new(planet.clone())),
                ObjectType::Asteroid(asteroid) => input_asteroids.push(asteroid.clone()),
        }
    );
    (planets, asteroids) = apply_physics(input_planets, input_asteroids, gravitational_constant);
    let mut circles:Vec<Circle> = vec![];
    planets.iter().for_each(|planet| circles.push(planet.as_circle() ));
    let contents = serde_json::to_string(&objects.iter().map(|o| o.get_circle() ).collect::<Vec<_>>()).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let response = format!(
        "{status_line}\r\nContentType: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{contents}\r\n"
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();

    objects
}

pub fn start_server(uri: &str, mut objects: Vec<ObjectType>, gravitational_constant: i32) -> ! {
    let listener = TcpListener::bind(uri).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        objects = handle_connection(stream, objects, gravitational_constant);
    }

    unreachable!()
}
