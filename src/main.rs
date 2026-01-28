pub(crate) use core::f32;
use math_3d::raytrace::{self, get_triangles};
use math_3d::{Material, Transform};
use sixel_rs::encoder::Encoder;
use sixel_sys::PixelFormat;
use std::io::{Write};
use std::io::stdout;

use crate::{
    frame_buffer::FrameBuffer,
    math_3d::{Point3d, Vec3},
};

mod cube;
mod frame_buffer;
mod math_3d;
mod penger;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;



fn clear_stdout() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1b[H");
    stdout().flush()?;
    Ok(())
}

fn flush_stdout() -> Result<(), Box<dyn std::error::Error>> {
    stdout().flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Lecture d'un modele wavefront
    let model = wavefront::Obj::from_file("sphere4.obj")?;
    dbg!(&model.triangles().count());

    #[allow(unreachable_code)]    
    let encoder: Encoder = match Encoder::new() {
        Ok(o) => o,
        Err(e) => {
            dbg!(e);
            let err = std::io::Error::other("Can't create Encoder");
            // return Err(err.into());
            return Err(Box::new(err));
        }
    };

    let mut fb: FrameBuffer = FrameBuffer::new(WIDTH, HEIGHT);
    fb.clean((255, 255, 255));
    clear_stdout()?;

    let mut z_buffer: Vec<f32> = vec![f32::NEG_INFINITY; WIDTH * HEIGHT];

    let target: Point3d = (0.0, 0.0, -0.0);
    let focal: f32 = WIDTH as f32 * 2.0;
    let eye: Point3d = (0.0, 0.0, 6.0);
    let light_dir: Vec3 = Vec3 {
        x: -0.5,
        y: 1.0,
        z: 1.0,
    }.normalize();

    let t1: math_3d::Transform = math_3d::Transform {
        scale: 1.0, 
        rotation: (std::f32::consts::PI / 2.0, 0.0, 0.0),
        translation: (0.0, 0.0, 0.0), 
    };

    let t2: math_3d::Transform = math_3d::Transform {
        scale: 1.0, 
        rotation: (0.0, std::f32::consts::PI, 0.0),
        translation: (0.0, 0.0, 0.0), 
    };
    
    fb.clean((0, 128, 255));
    clear_stdout()?;

    
    z_buffer.fill(f32::NEG_INFINITY);


    // let transforms: Vec<&Transform> = vec![&t1, &t2];
    let transforms: Vec<&Transform> = vec![];

    math_3d::utils::draw_obj_model_gouraud(&model, &transforms, &Material::white_plastic(), eye, target, light_dir, focal, WIDTH as u32, HEIGHT as u32, &mut fb, &mut z_buffer);


    // Récupération des triangles au format raytrace
    let all_triangles: Vec<(Vec3, Vec3, Vec3, Vec3, Vec3, Vec3)> = get_triangles(model.triangles());

    // Transformations
    let all_transformed_triangles: Vec<(Vec3, Vec3, Vec3, Vec3, Vec3, Vec3)> = raytrace::do_transforms(all_triangles, &transforms);

    // Render
    raytrace::render_raytrace(&all_transformed_triangles, eye, target, light_dir, WIDTH as u32, HEIGHT as u32, &mut fb);

    
    encoder
        .encode_bytes_ext(WIDTH, HEIGHT, &fb.pixels, PixelFormat::RGB888)
        .map_err(|e| {
            dbg!(e);
            std::io::Error::other("")
        })?;

    flush_stdout()?;

    // angle += 0.1;
    //}

    #[allow(unreachable_code)]
    Ok(())
}
