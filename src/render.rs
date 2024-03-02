use std::{ops::{self, Sub}, f64::consts::PI};
use rand::Rng; // 0.8.5

pub struct SurfacePoint {
    pub normal: Vector,
    pub color: Vector,
    pub position: Vector,
    pub is_light: bool,
    pub reflective: f64,
}

pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vector {
    pub fn dot(&self, b: &Vector) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn length(&self) -> f64 {
        return f64::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
    }

    pub fn add_(self, _rhs: &Vector) -> Vector {
        //println!("> Foo.add(Bar) was called");

        return Vector {x: self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z};
    }

    pub fn multiply(self, _rhs: f64) -> Vector {
        //println!("> Foo.add(Bar) was called");
        //println!("vector {} {} {} {}", self.x, self.y, self.z, _rhs);
        //println!("vector {} {} {}", self.x * _rhs, self.y * _rhs, )
        let res = Vector {x: self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs};
        //println!("{} {} {}", res.x, res.y, res.z);
        return res;
    }

    pub fn subtract(&self, _rhs: &Vector) -> Vector {
        //println!("> Foo.add(Bar) was called");

        return Vector {x: self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z};
    }

    // Check if correct
    pub fn cross(&self, b: &Vector) -> Vector {
        return Vector {
            x: self.y * b.z - self.z * b.y,
            y: self.x * b.z - self.z * b.x,
            z: self.x * b.y - self.y * b.x,
        };
    }

    pub fn normalize(&self) -> Vector {
        let l = self.length();
        return Vector {
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

}

impl ops::Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, _rhs: &Vector) -> Vector {
        return self.subtract(_rhs);
    }
}

impl ops::Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, _rhs: &Vector) -> Vector {
        return self.add_(_rhs);
    }
}

impl ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, _rhs: Vector) -> Vector {
        return self.add_(&_rhs);
    }
}

impl ops::Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, _rhs: f64) -> Vector {
        return self.multiply(_rhs);
    }
}
impl Copy for Vector {}
pub struct Spheres {
    pub positions: Vec<Vector>,
    pub sizes: Vec<f64>,
    pub colors: Vec<Vector>,
    pub is_light: Vec<bool>,
    pub reflective: Vec<f64>,
}

fn randomFloat() -> f64 {
    return rand::random()
}

pub fn render_recursive(ray: Ray, spheres: &Spheres, depth: i32) -> Vector {
    //println!("{} {} {}", ray.direction.x, ray.direction.y, ray.direction.z);

    if depth > 10 {
        //println!("returning fallback");
        return Vector {x: 1.0, y: 1.0, z: 1.0}
    }
    //println!("{}", depth);

    let intersect = find_intersection(&ray, &spheres);
    if intersect.is_none() {
        //println!("none");
        return Vector {x: 0.85, y: 0.85, z: 0.85};
    }
    let thing = intersect.unwrap();
    //println!("thing {} {} {}", thing.color.x, thing.color.y, thing.color.z);
    if thing.is_light {
        //println!("light");
        return thing.color;
    }
    //println!("running rec");
    //return thing.color;

    let newRay: Ray;
    let roulette = randomFloat();
    let reflect = roulette < thing.reflective;
    if reflect {
        newRay = generateReflectedRay(&thing.normal, &ray.direction, &thing.position);
    } else {
        newRay = generateNextRay(&thing.normal, &thing.position)
    }

    let rec_result = render_recursive(
        newRay, 
        spheres, 
        depth + 1);
    if reflect {
        return rec_result;
    }
    return Vector {
        x: rec_result.x * thing.color.x, 
        y: rec_result.y * thing.color.y, 
        z: rec_result.z * thing.color.z};
}

fn generateNextRay(normal: &Vector, position: &Vector) -> Ray {
    
    let rand_vector = Vector {x: randomFloat(), y: randomFloat(), z: randomFloat()}.normalize();
    let tangent = normal.cross(&rand_vector).normalize();

    let eps1 = randomFloat() * PI * 2.0;
    let eps2 = f64::sqrt(randomFloat());

    let x = f64::cos(eps1) * eps2;
    let y = f64::sin(eps1) * eps2;
    let z = f64::sqrt(1.0 - eps2*eps2);

    return Ray {origin: *position, direction: (tangent.multiply(x) + normal.cross(&tangent).multiply(y) + normal * z).normalize()};
}

fn generateReflectedRay(normal: &Vector, direction: &Vector, position: &Vector) -> Ray {
    let thing = normal.multiply(direction.dot(normal) * 2.0);

    return Ray { origin: *position, direction:  direction.sub(&thing).normalize()}
}

pub fn find_intersection(ray: &Ray, spheres: &Spheres) -> Option<SurfacePoint> {
    let mut smallestT = 1e10;
    let mut foundIndex: i32 = -1;
    let mut normal: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };
    let mut hit_pos: Vector = Vector { x: 0.0, y: 0.0, z: 0.0 };
    let mut color: Vector = Vector{x: 0.0, y: 0.0, z: 0.0};

    let nbr_spheres = spheres.positions.len();
    //println!("{} {}", smallestT, foundIndex);
    for sphere_index in 0..nbr_spheres {

        let l = spheres.positions[sphere_index].subtract(&ray.origin);
        //spheres.positions[sphere_index].subtract(&ray.origin);

        let tc = l.dot(&ray.direction);
         if tc < 0.0 {
            //println!("less than 0");
            continue;
         }

         let sphere_radius = spheres.sizes[sphere_index];
        let d = f64::sqrt(l.dot(&l) - tc*tc);
        //println!("{}", d);
        if d > sphere_radius {
            //println!("d > s");
            continue;
        }

        let t1c = f64::sqrt(sphere_radius* sphere_radius - d * d);
        let t1 = tc - t1c;
        let t2 = tc + t1c;

        let mut choice = 1e100;
        if t1 <= t2 && t1 > 0.001 {
            choice = t1;
        }
        if t2 <= t1 && t2 > 0.001 {
            choice = t2
        }

        if choice < smallestT {
            //println!("found");

            smallestT = choice;
            foundIndex = sphere_index as i32;
            let thing = &ray.direction.multiply(smallestT);
            hit_pos = ray.origin.add_(&ray.direction.multiply(smallestT));

            /*println!("raypos {} {} {}", ray.origin.x, ray.origin.y, ray.origin.z);
            println!("raydir {} {} {} {}", ray.direction.x, ray.direction.y, ray.direction.z, smallestT);
            println!("rayray {} {} {}", thing.x, thing.y, thing.z);

            println!("hitpos {} {} {}", hit_pos.x, hit_pos.y, hit_pos.z);*/

            normal = hit_pos.subtract(&spheres.positions[sphere_index]).normalize();
            if normal.dot(&ray.direction) > 0.0 {
                normal = Vector {x: -normal.x, y: -normal.y, z: -normal.z };
            }

            color = spheres.colors[sphere_index];
        }
        

    }
    if foundIndex != -1 {
        //println!("hit");
        return Some(SurfacePoint { 
            normal: normal, 
            color: color, 
            position: hit_pos, 
            is_light: spheres.is_light[foundIndex as usize],
            reflective: spheres.reflective[foundIndex as usize],
         })
    }
    return None;
}






