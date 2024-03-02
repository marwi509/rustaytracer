pub mod render;
use std::ops::Sub;
use std::ptr::null;
use std::{fs::File};
use std::io;
use std::io::Write;
use std::vec::Vec;
extern crate bmp;
use bmp::{Image, Pixel, px};
use core::ops;
use render::{Ray, Vector, Spheres};




fn clamp(x: f64, max: f64) -> f64 {
    if x > max {
        return max;
    }
    return x;
}

fn main() {
    const WIDTH: usize = 1920;
    const HEIGHT: usize = 1080;

//     let mut end_image = [0.5; WIDTH * HEIGHT * 3];
    let mut vec = vec![0.5; WIDTH * HEIGHT * 3];

    let spheres = Spheres {
        positions: vec![
            Vector {x: 0.0, y: -5.0, z: 50.0}, 
            Vector {x: -20.0, y: -5.0, z: 50.0}, 
            Vector {x: 50.0, y: 5.0, z: 80.0},
            Vector {x: 0.0, y: -10000.0 - 15.0, z: 1.0},
            //Vector {x: 0.0, y: 0.0, z: 0.0},
            ],
        sizes: vec![
            10.0, 
            10.0, 
            20.0,
            10000.0,
            //100000.0,
            ],
        colors: vec![
            Vector {x: 0.75, y: 0.55, z: 0.55},
            Vector {x: 0.55, y: 0.75, z: 0.55},
            Vector {x: 0.55, y: 0.55, z: 0.75},
            Vector {x: 0.75, y: 0.75, z: 0.75},
            //Vector {x: 1.0, y: 1.0, z: 0.0},
            ],
        is_light: vec![
            false,
            false,
            false,
            false,
            //true,
        ],
        reflective: vec![
            0.0,
            0.0,
            0.5,
            0.0,
        ],
    };


    println!("writing to file");
    //write_ppm_file(WIDTH, HEIGHT, &vec).expect("Fail");

    let mut img = Image::new(1920, 1080);

    let ar = WIDTH as f64 / HEIGHT as f64;
    for (x, y) in img.coordinates() {
        if x == 0 && y % 100 == 0 {
            println!("{}, {}", x, y);
        }
        let fx = ((x as f64 * 2.0 - WIDTH as f64) / WIDTH as f64);
        let fy = -((y as f64 * 2.0 - HEIGHT as f64) / HEIGHT as f64) / ar;
        let direction = Vector {x: fx, y: fy, z: 1.0}.normalize();
        let origin = Vector {x: fx, y: fy, z: 0.0};
        let ray = Ray {origin, direction};
        let samples = 200;
        let mut sample = Vector {x: 0.0, y: 0.0, z: 0.0};
        for i in 0..samples {
            let ray = Ray {origin, direction};
            let r_result = render::render_recursive(ray, &spheres, 0);
            //println!("{} {} {}", r_result.x, r_result.y, r_result.z);
            let spppp = r_result.multiply(255.0);
            //println!("{} {} {}", spppp.x, spppp.y, spppp.z);
            //println!("{} {} {}", 255.0 * r_result.x, 255.0 * r_result.y, 255.0 * r_result.z);
            sample = sample.add_(&spppp);
        }

        

        //println!("{} {} {}", sample.x, sample.y, sample.z);
        img.set_pixel(x, y, px!(
            clamp(sample.x / samples as f64, 255.0) as i32, 
            clamp(sample.y / samples as f64, 255.0) as i32, 
            clamp(sample.z / samples as f64, 255.0) as i32));
            /*img.set_pixel(x, y, px!(
                125, 
                125, 
                125));*/

        /*
               let sp = render::find_intersection(&ray, &spheres);
        if sp.is_some() {
            let spp = sp.unwrap();
            let light = Vector{x: -50.0, y: 10.0, z: -10.0};

            let dir_to_light = spp.position.subtract(&light);
            let dist = dir_to_light.length();
            let d = -spp.normal.dot(&dir_to_light.normalize());
            let color = spp.color;
            //println!("{} {} {}", normal.x, normal.y, normal.z);
            if d > 0.0 {
                
                let intens = (d * 100.0 * 1e4) / (dist * dist);//5e4 * 100.0 * (d / (dist * dist));
                let pp = px!((intens * color.x) as i32, (intens * color.y) as i32, (intens * color.z) as i32);
                //println!("adding color {}", intens);
                img.set_pixel(x, y, pp);
                //img.set_pixel(x, y, px!(200,200,200));
            }
        } else {
            img.set_pixel(x, y, px!(100,100,100));

        }*/
        
    }
    println!("saving");
    let _ = img.save("img.bmp");
}

