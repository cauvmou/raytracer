use raytracer::{camera::{Camera, Screen}, surface::Scene, meshes::Plane, materials::AlbedoMaterial};

fn main() {
    let camera = Camera::new(
        (100.0, 0.0, 2.0).into(),
        (0.0, 0.0, 2.0).into(),
        50.0,
        (0.0, 1.0, 0.0).into()
    );

    let mut scene: Scene = Vec::new();

    scene.push(Box::new(Plane::new(
        Box::new(AlbedoMaterial::new([0, 150, 255].into())),
        (0.0, 0.0, 0.0).into(),
        (0.0, 1.0, 2.0).into(),
    )));

    scene.push(Box::new(Plane::new(
        Box::new(AlbedoMaterial::new([255, 0, 0].into())),
        (0.0, 0.0, -2.0).into(),
        (0.0, -1.0, 1.0).into(),
    )));
    
    let mut screen = Screen::new(640, 480, 4.0, 3.0);

    screen = camera.render_scene(&scene, None, screen);
    screen.export("./out.png").expect("Failed to save image.");
}
