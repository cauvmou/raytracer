use camera::{Camera, Screen};
use materials::AlbedoMaterial;
use math::Vec3;
use meshes::Plane;
use surface::Scene;


mod camera;
mod math;
mod surface;
mod materials;
mod meshes;

fn main() {
    let camera = Camera::new(
        (100.0, 0.0, 2.0).into(),
        (0.0, 0.0, 2.0).into(),
        20.0,
        (0.0, 0.0, 1.0).into()
    );

    let mut scene: Scene = Vec::new();
    scene.push(Box::new(Plane {
        material: Box::new(AlbedoMaterial::new(255, 0, 0)),
        point: (0.0, 0.0, 0.0).into(),
        normal: (0.0, 0.0, 1.0).into(),
    }));
    
    let mut screen = Screen::new(1920, 1080, 16.0, 9.0);

    screen = camera.render_scene(&scene, screen);
    screen.export("/home/cauvmou/programming/raytracer/out.png").expect("Failed to save image.");
}
