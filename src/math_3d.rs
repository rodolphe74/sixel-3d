#[allow(dead_code)]
pub type Point2d = (i32, i32);

#[allow(dead_code)]
pub type Point3d = (f32, f32, f32);

pub trait ToPoint3d {
    fn to_p3d(self) -> Point3d;
}

impl ToPoint3d for (f64, f64, f64) {
    fn to_p3d(self) -> Point3d {
        (self.0 as f32, self.1 as f32, self.2 as f32)
    }
}

#[allow(dead_code)]
pub type Color = (u8, u8, u8);

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
pub struct Vertex {
    pub pos: Vec3,    // Utilise ta structure Vec3 existante (x, y, z)
    pub color: Color, // Couleur associée à ce sommet
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn new_from_point3d(p: Point3d) -> Self {
        let x = p.0;
        let y = p.1;
        let z = p.2;
        Self { x, y, z }
    }

    fn from(p: Point3d) -> Self {
        Vec3 {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }

    // Addition de deux vecteurs
    pub fn add(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    // Soustraction (Très utilisé pour P - Eye)
    pub fn sub(&self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    // Produit scalaire (Dot Product) : donne la projection d'un vecteur sur un autre
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn neg(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    // Produit vectoriel (Cross Product) : crée un vecteur perpendiculaire à deux autres
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    // Longueur du vecteur
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Normalisation : rend le vecteur unitaire (longueur = 1)
    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len > 0.0 {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}


// Pour faciliter l'interpolation dans draw_triangle_shaded
#[allow(dead_code)]
struct ProjectPoint {
    x: f32,
    y: f32,
    z: f32,
    r: f32,
    g: f32,
    b: f32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Vec3WithColor {
    pub x: f32, // Coordonnée écran
    pub y: f32, // Coordonnée écran
    pub z: f32, // Profondeur pour le Z-Buffer
    pub c: Color, // pub r: f32,
    // pub g: f32,
    // pub b: f32,
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Material {
    pub ka: (f32, f32, f32), // Ambiant RGB
    pub kd: (f32, f32, f32), // Diffus RGB
    pub ks: (f32, f32, f32), // Spéculaire RGB
    pub ns: f32,             // Shininess
}

#[allow(dead_code)]
impl Material {
    // --- PIERRES PRÉCIEUSES ---
    pub fn emerald() -> Self {
        Self { ka: (0.0215, 0.1745, 0.0215), kd: (0.07568, 0.61424, 0.07568), ks: (0.633, 0.72781, 0.633), ns: 76.8 }
    }
    pub fn jade() -> Self {
        Self { ka: (0.135, 0.2225, 0.1575), kd: (0.54, 0.89, 0.63), ks: (0.31622, 0.31622, 0.31622), ns: 12.8 }
    }
    pub fn obsidian() -> Self {
        Self { ka: (0.05375, 0.05, 0.06625), kd: (0.18275, 0.17, 0.22525), ks: (0.33274, 0.32863, 0.34643), ns: 38.4 }
    }
    pub fn pearl() -> Self {
        Self { ka: (0.25, 0.20725, 0.20725), kd: (1.0, 0.829, 0.829), ks: (0.29665, 0.29665, 0.29665), ns: 11.2 }
    }
    pub fn ruby() -> Self {
        Self { ka: (0.1745, 0.01175, 0.01175), kd: (0.61424, 0.04136, 0.04136), ks: (0.72781, 0.62696, 0.62696), ns: 76.8 }
    }
    pub fn turquoise() -> Self {
        Self { ka: (0.1, 0.18725, 0.1745), kd: (0.396, 0.74151, 0.69102), ks: (0.29774, 0.30829, 0.30668), ns: 12.8 }
    }

    // --- MÉTAUX ---
    pub fn brass() -> Self {
        Self { ka: (0.32941, 0.22353, 0.02745), kd: (0.78039, 0.56863, 0.11373), ks: (0.99216, 0.94118, 0.80784), ns: 27.9 }
    }
    pub fn bronze() -> Self {
        Self { ka: (0.2125, 0.1275, 0.054), kd: (0.714, 0.4284, 0.18144), ks: (0.39355, 0.27191, 0.16672), ns: 25.6 }
    }
    pub fn chrome() -> Self {
        Self { ka: (0.25, 0.25, 0.25), kd: (0.4, 0.4, 0.4), ks: (0.77459, 0.77459, 0.77459), ns: 76.8 }
    }
    pub fn copper() -> Self {
        Self { ka: (0.19125, 0.0735, 0.0225), kd: (0.7038, 0.27048, 0.0828), ks: (0.25678, 0.13762, 0.08601), ns: 12.8 }
    }
    pub fn gold() -> Self {
        Self { ka: (0.24725, 0.1995, 0.0745), kd: (0.75164, 0.60648, 0.22648), ks: (0.62828, 0.5558, 0.36606), ns: 51.2 }
    }    
    pub fn silver() -> Self {
        Self { ka: (0.19225, 0.19225, 0.19225), kd: (0.50754, 0.50754, 0.50754), ks: (0.50827, 0.50827, 0.50827), ns: 51.2 }
    }

    // --- PLASTIQUES ---
    pub fn black_plastic() -> Self {
        Self { ka: (0.0, 0.0, 0.0), kd: (0.01, 0.01, 0.01), ks: (0.5, 0.5, 0.5), ns: 32.0 }
    }
    pub fn cyan_plastic() -> Self {
        Self { ka: (0.0, 0.1, 0.06), kd: (0.0, 0.5098, 0.5098), ks: (0.502, 0.502, 0.502), ns: 32.0 }
    }
    pub fn green_plastic() -> Self {
        Self { ka: (0.0, 0.0, 0.0), kd: (0.1, 0.35, 0.1), ks: (0.45, 0.55, 0.45), ns: 32.0 }
    }
    pub fn red_plastic() -> Self {
        Self { ka: (0.0, 0.0, 0.0), kd: (0.5, 0.0, 0.0), ks: (0.7, 0.6, 0.6), ns: 32.0 }
    }
    pub fn white_plastic() -> Self {
        Self { ka: (0.0, 0.0, 0.0), kd: (0.55, 0.55, 0.55), ks: (0.7, 0.7, 0.7), ns: 32.0 }
    }
    pub fn yellow_plastic() -> Self {
        Self { ka: (0.0, 0.0, 0.0), kd: (0.5, 0.5, 0.0), ks: (0.6, 0.6, 0.5), ns: 32.0 }
    }

    // --- CAOUTCHOUC ---
    pub fn black_rubber() -> Self {
        Self { ka: (0.02, 0.02, 0.02), kd: (0.01, 0.01, 0.01), ks: (0.4, 0.4, 0.4), ns: 10.0 }
    }
    pub fn cyan_rubber() -> Self {
        Self { ka: (0.0, 0.05, 0.05), kd: (0.4, 0.5, 0.5), ks: (0.04, 0.7, 0.7), ns: 10.0 }
    }
    pub fn green_rubber() -> Self {
        Self { ka: (0.0, 0.05, 0.0), kd: (0.4, 0.5, 0.4), ks: (0.04, 0.7, 0.04), ns: 10.0 }
    }
    pub fn red_rubber() -> Self {
        Self { ka: (0.05, 0.0, 0.0), kd: (0.5, 0.4, 0.4), ks: (0.7, 0.04, 0.04), ns: 10.0 }
    }
    pub fn white_rubber() -> Self {
        Self { ka: (0.05, 0.05, 0.05), kd: (0.5, 0.5, 0.5), ks: (0.7, 0.7, 0.7), ns: 10.0 }
    }
    pub fn yellow_rubber() -> Self {
        Self { ka: (0.05, 0.05, 0.0), kd: (0.5, 0.5, 0.4), ks: (0.7, 0.7, 0.04), ns: 10.0 }
    }
}


pub struct Transform {
    pub scale: f32,
    pub rotation: (f32, f32, f32), // (angle_x, angle_y, angle_z)
    pub translation: (f32, f32, f32),
}

#[allow(dead_code)]
impl Transform {
    pub fn transform_point(p: Point3d, t: &Transform) -> Point3d {
        let (mut x, mut y, mut z) = p;

        // 1. SCALE (Agrandir / Réduire)
        x *= t.scale;
        y *= t.scale;
        z *= t.scale;

        // 2. ROTATIONS (L'ordre Y -> X -> Z est souvent le plus naturel)
        // Rotation Y
        let (cos_y, sin_y) = (t.rotation.1.cos(), t.rotation.1.sin());
        let x_y = x * cos_y + z * sin_y;
        let z_y = -x * sin_y + z * cos_y;
        x = x_y;
        z = z_y;

        // Rotation X
        let (cos_x, sin_x) = (t.rotation.0.cos(), t.rotation.0.sin());
        let y_x = y * cos_x - z * sin_x;
        let z_x = y * sin_x + z * cos_x;
        y = y_x;
        z = z_x;

        // Rotation Z
        let (cos_z, sin_z) = (t.rotation.2.cos(), t.rotation.2.sin());
        let x_z = x * cos_z - y * sin_z;
        let y_z = x * sin_z + y * cos_z;
        x = x_z;
        y = y_z;

        // 3. TRANSLATION (Déplacer dans l'espace)
        x += t.translation.0;
        y += t.translation.1;
        z += t.translation.2;

        (x, y, z)
    }

    pub fn transform_vec3(v: Vec3, t: &Transform) -> Vec3 {
        let (mut x, mut y, mut z) = (v.x, v.y, v.z);

        // 1. SCALE (Optionnel pour une normale, mais on le garde si c'est uniforme)
        x *= t.scale;
        y *= t.scale;
        z *= t.scale;

        // 2. ROTATIONS (On applique les mêmes que pour les points)
        // Rotation Y
        let (cos_y, sin_y) = (t.rotation.1.cos(), t.rotation.1.sin());
        let x_y = x * cos_y + z * sin_y;
        let z_y = -x * sin_y + z * cos_y;
        x = x_y;
        z = z_y;

        // Rotation X
        let (cos_x, sin_x) = (t.rotation.0.cos(), t.rotation.0.sin());
        let y_x = y * cos_x - z * sin_x;
        let z_x = y * sin_x + z * cos_x;
        y = y_x;
        z = z_x;

        // Rotation Z
        let (cos_z, sin_z) = (t.rotation.2.cos(), t.rotation.2.sin());
        let x_z = x * cos_z - y * sin_z;
        let y_z = x * sin_z + y * cos_z;
        x = x_z;
        y = y_z;

        // --- ATTENTION : ON NE FAIT PAS LA TRANSLATION ICI ---

        // 3. NORMALISATION
        // Très important : après une rotation/scale, on s'assure que
        // la normale fait toujours 1.0 de long pour que le produit scalaire soit juste.
        Vec3::new(x, y, z).normalize()
    }
}

#[allow(dead_code)]
// Dans ce repère (souvent appelé View Space ou Camera Space) :
//     L'œil (l'origine) : Il est toujours à (0,0,0).
//     L'axe Z (Forward) : C'est la ligne droite qui part de l'oeil et traverse le centre de l'écran.
//     Le Plan de Projection : C'est une surface plane située à la coordonnée z=focal.
//     La Focale : C'est la distance (en unités de ce repère) entre l'origine (0,0,0) et ce plan.
//     Les paramètres sont donnés dans le repère Monde (et donc pas le repère de l'oeil)
//     Il y a un changement de repère du Monde à celui de l'oeil (caméra)
//     Puis une projection
pub mod utils {

    use std::usize;

    use wavefront::Obj;

    use crate::frame_buffer::{self, FrameBuffer};
    use crate::math_3d::{self, Point3d, Vec3, Vec3WithColor};

    use super::{Color, Material};

    pub fn project_look_at(
        p: Point3d,
        eye: Point3d,
        target: Point3d,
        focal: f32,
        width: f32,
        height: f32,
    ) -> Option<(i32, i32, f32)> {
        // Ajout du f32 pour le Z-buffer
        // 1. Calcul des axes du repère
        let world_up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let eye = Vec3::from(eye);
        let target = Vec3::from(target);
        let f = (target.sub(eye)).normalize();
        let r = f.cross(world_up).normalize();
        let u = r.cross(f);

        // 2. Passage au repère caméra
        let p = Vec3::from(p);
        let v = p.sub(eye);
        let x_cam = v.dot(r);
        let y_cam = v.dot(u);
        let z_cam = v.dot(f);

        // 3. Clipping Z (Near Plane)
        // Sécurité absolue contre la division par zéro et les points derrière la caméra
        if z_cam <= 0.1 {
            return None;
        }

        // 4. Projection
        let px = (x_cam * focal) / z_cam;
        let py = (y_cam * focal) / z_cam;

        // --- SÉCURITÉ ANTI-CRASH (Clipping 2D) ---
        // Si px ou py sont délirants (ex: 35000), on rejette le point avant le cast en i32.
        // Cela évite l'overflow dans les calculs de lignes/surfaces de la bibliothèque.
        let limit = 10000.0;
        if px.abs() > limit || py.abs() > limit {
            return None;
        }

        // Centrage sur le FrameBuffer
        let screen_x = (px + width / 2.0) as i32;
        let screen_y = (height / 2.0 - py) as i32;

        // --- LE Z POUR LE BUFFER ---
        // On renvoie 1.0 / z_cam.
        // Valeur la plus grande est plus proche
        // Ainsi : z_cam = 0.1 (proche) -> 10.0 (grand Z)
        //         z_cam = 100.0 (loin) -> 0.01 (petit Z)
        // Cela correspond parfaitement à ton test "if new_z > z_buffer".
        Some((screen_x, screen_y, 1.0 / z_cam)) // profondeur w
    }

    pub fn is_point_in_polygon(poly: &[(f32, f32)], t: (f32, f32)) -> bool {
        let mut c = false;
        let nvert = poly.len();
        let mut j = nvert - 1;

        for i in 0..nvert {
            let pi = poly[i];
            let pj = poly[j];

            // Vérifie si le point est entre les niveaux Y des deux sommets
            // ET si le point est à gauche de l'arête (intersection du rayon)
            if ((pi.1 > t.1) != (pj.1 > t.1))
                && (t.0 < (pj.0 - pi.0) * (t.1 - pi.1) / (pj.1 - pi.1) + pi.0)
            {
                c = !c;
            }
            j = i;
        }
        c
    }

    pub fn calculate_normal(p0: Point3d, p1: Point3d, p2: Point3d) -> Vec3 {
        let a = Vec3::from(p0);
        let b = Vec3::from(p1);
        let c = Vec3::from(p2);

        // Calcul des deux vecteurs d'arêtes partant du même sommet
        let v1 = b.sub(a);
        let v2 = c.sub(a);

        // Produit vectoriel pour obtenir la perpendiculaire (Normale)
        // L'ordre (v1 cross v2) définit si la normale sort ou rentre
        v1.cross(v2).normalize()
    }

    pub fn compute_vertex_normals(points: &[(f64, f64, f64)], faces: &[&[usize]]) -> Vec<Vec3> {
        let mut normals = vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };
            points.len()
        ];

        for f in faces {
            if f.len() < 3 {
                continue;
            }

            // Calcul de la normal de la face à partir des 3 premiers points
            let p0 = Vec3::new_from_point3d(math_3d::ToPoint3d::to_p3d(points[f[0]]));
            let p1 = Vec3::new_from_point3d(math_3d::ToPoint3d::to_p3d(points[f[1]]));
            let p2 = Vec3::new_from_point3d(math_3d::ToPoint3d::to_p3d(points[f[2]]));
            // let face_normal = p1.sub(p0).cross(p2.sub(p0));
            let face_normal = p2.sub(p0).cross(p1.sub(p0));

            // Distribue la normal à tous les points sommets de la face
            for vertex_idx in *f {
                let i: usize = *vertex_idx;
                normals[i] = face_normal;
            }
        }

        normals
    }



    pub fn draw_triangle_shaded(
        p: [Vec3WithColor; 3],
        fb: &mut frame_buffer::FrameBuffer,
        z_buffer: &mut [f32],
        width: i32,
        height: i32,
    ) {
        // 1. Tes limites sont déjà bien gérées avec .round()
        let min_x = p.iter().map(|v| v.x).fold(f32::INFINITY, f32::min).round() as i32;
        let max_x = p.iter().map(|v| v.x).fold(f32::NEG_INFINITY, f32::max).round() as i32;
        let min_y = p.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).round() as i32;
        let max_y = p.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).round() as i32;

        let start_x = min_x.clamp(0, width - 1);
        let end_x = max_x.clamp(0, width - 1);
        let start_y = min_y.clamp(0, height - 1);
        let end_y = max_y.clamp(0, height - 1);

        let den = (p[1].y - p[2].y) * (p[0].x - p[2].x) + (p[2].x - p[1].x) * (p[0].y - p[2].y);
        if den.abs() < 1e-6 { return; }

        let epsilon = -0.0001; // Marge pour boucher les trous entre faces
        for y in start_y..=end_y {
            for x in start_x..=end_x {
                // On teste le CENTRE du pixel (+ 0.5) pour la précision
                let fx = x as f32 + 0.5; 
                let fy = y as f32 + 0.5;

                let w0 = ((p[1].y - p[2].y) * (fx - p[2].x) + (p[2].x - p[1].x) * (fy - p[2].y)) / den;
                let w1 = ((p[2].y - p[0].y) * (fx - p[2].x) + (p[0].x - p[2].x) * (fy - p[2].y)) / den;
                let w2 = 1.0 - w0 - w1;

                // Utilisation de l'epsilon au lieu de 0.0
                if w0 >= epsilon && w1 >= epsilon && w2 >= epsilon {
                    let z = p[0].z * w0 + p[1].z * w1 + p[2].z * w2;
                    let offset = (y * width + x) as usize;

                    if z >= z_buffer[offset] {
                        // Interpolation des couleurs (inchangée)
                        let r = (p[0].c.0 as f32 * w0 + p[1].c.0 as f32 * w1 + p[2].c.0 as f32 * w2) as u8;
                        let g = (p[0].c.1 as f32 * w0 + p[1].c.1 as f32 * w1 + p[2].c.1 as f32 * w2) as u8;
                        let b = (p[0].c.2 as f32 * w0 + p[1].c.2 as f32 * w1 + p[2].c.2 as f32 * w2) as u8;

                        z_buffer[offset] = z;
                        fb.pixel(x as u32, y as u32, (r, g, b));
                    }
                }
            }
        }
    }


    pub fn draw_object_model(model: &wavefront::Object,
                             transforms: &Vec<&math_3d::Transform>,
                             material: &Material,
                             eye: Point3d,
                             target: Point3d,
                             light_dir: Vec3,
                             focal: f32,
                             width: u32, height: u32,
                             fb: &mut FrameBuffer,
                             z_buffer: &mut Vec<f32>) {

        let eye_vec = Vec3::new_from_point3d(eye);
        
        for f in model.triangles() {
            let fsz: usize = f.len();

            if fsz < 3 {
                return;
            }

            let mut world_points: Vec<Point3d> = Vec::with_capacity(fsz);
            let mut world_normals: Vec<Vec3> = Vec::with_capacity(fsz);

            for v in f {
                let p: Point3d = (v.position()[0], v.position()[1], v.position()[2]);

                // == Gestion des transformations points ==
                if transforms.len() > 0 { 
                    let mut pt = math_3d::Transform::transform_point(p, &transforms[0]);
                    for t in transforms.iter().skip(1) {
                        pt = math_3d::Transform::transform_point(pt, t);
                    }
                    world_points.push(pt);
                } else {
                    world_points.push(p);
                }

                // == Gestion des transformations vecteurs normales ==
                let n = if let Some(n_exists) = v.normal() {
                    n_exists
                } else {
                    // Todo calculer la normale
                    [0.0, 1.0, 0.0]
                };

                if transforms.len() > 0 {
                    let mut vn = Vec3 { x: n[0], y: n[1], z: n[2] };
                    vn = math_3d::Transform::transform_vec3(vn, transforms[0]);
                    for t in transforms.iter().skip(1) {
                        vn = math_3d::Transform::transform_vec3(vn, t);
                    }
                    world_normals.push(vn);
                } else {
                    world_normals.push(Vec3 { x: n[0], y: n[1], z: n[2] });
                }
            }

            // == Back-face culling ==
            let p0 = world_points[0];
            let p1 = world_points[1];
            let p2 = world_points[2];
            let normal = math_3d::utils::calculate_normal(p0, p1, p2);
            let vec_p0 = Vec3::new_from_point3d(p0);
            // Vecteur allant du triangle vers la caméra
            // 2 points A et B dans l'espace, le vecteur AB s'obtient par AB = B - A
            let view_dir = eye_vec.sub(vec_p0);
            // Si le produit scalaire est négatif, la face regarde ailleurs
            // Angle θ < 90° (positif)  ou Angle θ > 90° (négatif) entre normale et vue 
            if normal.dot(view_dir) <= 0.0 {
                continue;
            }

            // == Shading ==

            // Lambert :   dot_light = N • L = |N| |L| cos(θ) = cos(θ)
            // θ est l'angle entre la normal et la direction de la lumière
            // max(0) : pas d'éclairage si lumière derrière la surface

            // Gestion des matériaux
            // https://opengl.developpez.com/tutoriels/apprendre-opengl-2/?page=materiaux
            // Ka	Ambiant	La couleur de l'objet dans l'ombre (ici 80% de gris).
            // Kd	Diffus	La couleur principale (Lambert) sous la lumière directe.
            // Ks	Spéculaire	La couleur du reflet brillant (ici blanc/gris clair).
            // Ns	Shininess	L'exposant de brillance (500 est très élevé = reflet très net et "métallique").
            // illum 2	Modèle	Indique qu'il faut utiliser le modèle Phong (Diffus + Spéculaire).
            // Color=(Ka×Ambiant)+(Kd×Diffus)+(Ks×Speculaire)
            // 1. Définir les propriétés du matériau (à extraire de ton .mtl)

            // let m = math_3d::Material::gold();
            let mut vertex_colors: Vec<Color> = Vec::with_capacity(world_normals.len());
            // On utilise .zip() pour avoir le point ET la normale correspondante
            for (p_world, n_world) in world_points.iter().zip(world_normals.iter()) {
                
                // 1. Vecteurs de base
                let l = light_dir; 
                let v = eye_vec.sub(Vec3::new_from_point3d(*p_world)).normalize();
                let n = n_world; // Garde n_world.neg() si l'objet est noir
                // let n = n_world.neg();  // parfois la normale a besoin d'être inversée
                
                // 2. Pré-calculs des produits scalaires
                let dot_diffuse = n.dot(l).max(0.0);
                
                let h = l.add(v).normalize();
                let dot_spec = n.dot(h).max(0.0);
                let spec_power = dot_spec.powf(material.ns);

                // 3. Calcul de l'intensité par canal RGB
                // On multiplie chaque composante du matériau (ka, kd, ks) par son intensité lumineuse
                // Note : On utilise un multiplicateur de 1.0 pour l'ambiant, 
                // mais tu peux le baisser si le rendu est trop clair.
                
                let r_int = (material.ka.0 * 1.0) + (material.kd.0 * dot_diffuse) + (material.ks.0 * spec_power);
                let g_int = (material.ka.1 * 1.0) + (material.kd.1 * dot_diffuse) + (material.ks.1 * spec_power);
                let b_int = (material.ka.2 * 1.0) + (material.kd.2 * dot_diffuse) + (material.ks.2 * spec_power);

                // 4. Application de la couleur finale
                // Si 'c' était ta couleur de base, on l'utilise comme filtre (ou on utilise directement 255)
                let color = (
                    (255.0 * r_int).min(255.0) as u8,
                    (255.0 * g_int).min(255.0) as u8,
                    (255.0 * b_int).min(255.0) as u8,
                );
                
                vertex_colors.push(color);
            }

            // Projection des sommets et des couleurs
            let mut projected_vertices: Vec<Vec3WithColor> = Vec::with_capacity(f.len());
            let mut all_visible = true;
            for (p, &color) in world_points.iter().zip(vertex_colors.iter()) {
                if let Some((sx, sy, sz)) = math_3d::utils::project_look_at(
                    *p,
                    eye,
                    target,
                    focal,
                    width as f32,
                    height as f32,
                ) {
                    projected_vertices.push(Vec3WithColor { x: sx as f32, y: sy as f32, z: sz, c: color });
                } else {
                    all_visible = false;
                    break;
                }
            }

            // Triangulation et dessin
            if all_visible && projected_vertices.len() >= 3 {
                for i in 1..projected_vertices.len() - 1 {
                    let triangle: [Vec3WithColor; 3] = [
                        projected_vertices[0],
                        projected_vertices[i],
                        projected_vertices[i + 1],
                    ];
                    draw_triangle_shaded(
                        triangle,
                        fb,
                        z_buffer,
                        width as i32,
                        height as i32,
                    );
                }
            }            
        }
    }



    pub fn draw_obj_model(model: &Obj,
                          transforms: &Vec<&math_3d::Transform>,
                          material: &Material,
                          eye: Point3d,
                          target: Point3d,
                          light_dir: Vec3,
                          focal: f32,
                          width: u32, height: u32,
                          fb: &mut FrameBuffer,
                          z_buffer: &mut Vec<f32>) {
        for o in model.objects() {
            let object = o.1;
            draw_object_model(&object, transforms, material, eye, target, light_dir, focal, width, height, fb, z_buffer);
        }
    }

}
