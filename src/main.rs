pub(crate) use core::f32;
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
    let model = wavefront::Obj::from_file("torso_slayer.obj")?;
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
    let eye: Point3d = (0.0, 0.0, 210.0);
    let light_dir: Vec3 = Vec3 {
        x: 0.5,
        y: 0.5,
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


    let transforms: Vec<&Transform> = vec![&t1, &t2];
    // let transforms: Vec<&Transform> = vec![];

    
    math_3d::utils::draw_obj_model(&model, &transforms, &Material::green_plastic(),  eye, target, light_dir, focal, WIDTH as u32, HEIGHT as u32, &mut fb, &mut z_buffer);

    
    // for f in penger::FACES {
    //     let fsz: usize = f.len();

    //     if fsz < 3 {
    //         continue;
    //     }

    //     // 0. Transformation
    //     let mut world_points: Vec<Point3d> = Vec::with_capacity(fsz);
    //     let mut world_normals: Vec<Vec3> = Vec::with_capacity(fsz);
    //     for idx in &f {
    //         // Transformation du point
    //         let p: Point3d = penger::POINTS[*idx].to_p3d();
    //         world_points.push(math_3d::Transform::transform_point(p, &t));

    //         // Transformation de la normale
    //         let n_local = normals[*idx];
    //         let n_world = math_3d::Transform::transform_vec3(n_local, &t);
    //         world_normals.push(n_world);
    //     }

    //     // 1. Calcul de la normale sur les 3 premiers points
    //     let p0 = world_points[0];
    //     let p1 = world_points[1];
    //     let p2 = world_points[2];

    //     let normal = math_3d::utils::calculate_normal(p0, p1, p2);
    //     let vec_normal = Vec3::new_from_point3d(p0);

    //     // 2. Back-face culling
    //     // Vecteur allant du triangle vers la caméra
    //     // 2 points A et B dans l'espace, le vecteur AB s'obtient par : AB = B - A
    //     let view_dir = eye_vec.sub(vec_normal).normalize();

    //     // Si le produit scalaire est négatif, la face "regarde" ailleurs
    //     if normal.dot(view_dir) <= 0.0 {
    //         continue;
    //     };

    //     // 3. Shading
    //     // Lambert :   dot_light = N • L = |N| |L| cos(θ) = cos(θ)
    //     // θ est l'angle entre la normal et la direction de la lumière
    //     // max(0) : pas d'éclairage si lumière derrière la surface

    //     let mut vertex_colors: Vec<Color> = Vec::with_capacity(world_normals.len());
    //     for n in world_normals {
    //         let dot_light = n.dot(light_dir);
    //         // let dot_light = n.neg().dot(light_dir).max(0.0);
    //         let intensity = 0.2 + 0.8 * dot_light;
    //         let color = (
    //             (200.0 * intensity) as u8,
    //             (200.0 * intensity) as u8,
    //             (200.0 * intensity) as u8,
    //         );
    //         vertex_colors.push(color);
    //     }

    //     // 4. Projection des sommets et des couleurs en parallele
    //     let mut projected_vertices: Vec<Vec3WithColor> = Vec::with_capacity(f.len());
    //     let mut all_visible = true;

    //     for (p, &color) in world_points.iter().zip(vertex_colors.iter()) {
    //         if let Some((sx, sy, sz)) = math_3d::utils::project_look_at(
    //             *p,
    //             eye,
    //             target,
    //             focal,
    //             WIDTH as f32,
    //             HEIGHT as f32,
    //         ) {
    //             projected_vertices.push(Vec3WithColor {
    //                 x: sx as f32,
    //                 y: sy as f32,
    //                 z: sz,
    //                 c: color,
    //             });
    //         } else {
    //             all_visible = false;
    //             break;
    //         }
    //     }

    //     // 5. Triangulation et dessin
    //     if all_visible && projected_vertices.len() >= 3 {
    //         for i in 1..projected_vertices.len() - 1 {
    //             let triangle: [Vec3WithColor; 3] = [
    //                 projected_vertices[0],
    //                 projected_vertices[i],
    //                 projected_vertices[i + 1],
    //             ];
    //             draw_triangle_shaded(
    //                 triangle,
    //                 &mut fb,
    //                 &mut z_buffer,
    //                 WIDTH as i32,
    //                 HEIGHT as i32,
    //             );
    //         }
    //     }
    // }
    
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
