use std::io::{Write, stdout};
use frame_buffer::FrameBuffer;
use math_3d::{Point3d, Transform, Vec3};
use math_3d::raytrace::{self, get_obj};
use math_3d::MaterialRaytrace;
use sixel_rs::encoder::Encoder;
use sixel_sys::PixelFormat;
mod math_3d;
mod frame_buffer;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

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


    let mut fb: FrameBuffer = FrameBuffer::new(WIDTH, HEIGHT);
    fb.clean((255, 255, 255));
    clear_stdout()?;

    let model = wavefront::Obj::from_file("torso_slayer.obj")?;
    dbg!(&model.triangles().count());
    let target: Point3d = (0.0, 0.0, -0.0);
    let eye: Point3d = (0.0, 0.0, 210.0);
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

    let transforms: Vec<&Transform> = vec![&t1, &t2];
    let material: MaterialRaytrace = MaterialRaytrace::polished_steel_pov();
    let mut slayer_obj = get_obj(wavefront::Obj::from_file("torso_slayer.obj")?.triangles(), &material);
    slayer_obj.do_transforms(&transforms);

    let scene = raytrace::Scene::build(vec![slayer_obj]);
    raytrace::render_scene_raytrace(&scene, eye, target, light_dir, WIDTH as u32, HEIGHT as u32, &mut fb);

    let encoder: Encoder = match Encoder::new() {
        Ok(o) => o,
        Err(e) => {
            dbg!(e);
            let err = std::io::Error::other("Can't create Encoder");
            return Err(Box::new(err));
        }
    };
    encoder
        .encode_bytes_ext(WIDTH, HEIGHT, &fb.pixels, PixelFormat::RGB888)
        .map_err(|e| {
            dbg!(e);
            std::io::Error::other("")
        })?;

    flush_stdout()?;
    
    Ok(())
}
