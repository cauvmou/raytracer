use raytracer::{camera::{Camera, Screen}, surface::Scene, meshes::{Plane, Sphere}, materials::AlbedoMaterial, light::{SceneLights, DirectionalLight}};

fn main() {
    let camera = Camera::new(
        (100.0, 0.0, 50.0).into(),
        (0.0, 0.0, 5.0).into(),
        20.0,
        (0.0, 0.0, 1.0).into()
    );

    let mut scene: Scene = Vec::new();
    let mut lights: SceneLights = Vec::new();

    scene.push(Box::new(Plane::new(
        Box::new(AlbedoMaterial::new([0, 0, 255].into())),
        (0.0, 0.0, 0.0).into(),
        (0.0, 0.0, 1.0).into(),
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(AlbedoMaterial::new([0, 255, 0].into())),
        (0.0, -3.5, 1.5).into(),
        2.0,
    )));
    scene.push(Box::new(Sphere::new(
        Box::new(AlbedoMaterial::new([255, 255, 0].into())),
        (0.0, 3.0, 0.0).into(),
        2.0,
    )));
    scene.push(Box::new(Sphere::new(
        Box::new(AlbedoMaterial::new([200, 0, 100].into())),
        (-5.0, 0.0, 6.5).into(),
        3.0,
    )));

    lights.push(Box::new(DirectionalLight::new((1.0, 1.0, 1.0).into(), [255, 255, 255].into(), 1.0)));
    lights.push(Box::new(DirectionalLight::new((1.0, -1.0, 1.0).into(), [255, 255, 255].into(), 1.0)));
    
    let mut screen = Screen::new(640, 480, 4.0, 3.0);

    screen = camera.render_scene(&scene, Some(&lights), screen);
    screen.export("./out/03out.png").expect("Failed to save image.");
}
