use raytracer::{camera::{Camera, Screen}, surface::Scene, meshes::{Plane, Sphere}, materials::{DiffuseMaterial}, light::{SceneLights, DirectionalLight, PointLight}};

fn main() {
    let camera = Camera::new(
        (100.0, 100.0, 30.0).into(),
        (0.0, 0.0, 5.0).into(),
        20.0,
        (0.0, 0.0, 1.0).into()
    );

    let mut scene: Scene = Vec::new();
    let mut lights: SceneLights = Vec::new();

    scene.push(Box::new(Plane::new(
        Box::new(DiffuseMaterial::new([0, 0, 255].into(), 0.5)),
        (0.0, 0.0, 0.0).into(),
        (0.0, 0.0, 1.0).into(),
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(DiffuseMaterial::new([255, 0, 0].into(), 0.6)),
        (0.0, 0.0, 4.0).into(),
        3.0,
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(DiffuseMaterial::new([20, 255, 20].into(), 0.6)),
        (-5.0, 10.0, 6.0).into(),
        3.0,
    )));

    lights.push(Box::new(PointLight::new((0.0, 10.0, 4.0).into(), [255, 255, 255].into(), 5.0)));
    lights.push(Box::new(PointLight::new((0.0, 10.0, 4.0).into(), [255, 255, 255].into(), 50.0).with_falloff()));
    lights.push(Box::new(DirectionalLight::new((0.0, 10.0, 4.0).into(), [255, 255, 255].into(), 5.0)));
    
    let mut screen = Screen::new(640, 480, 4.0, 3.0);

    screen = camera.render_scene(&scene, Some(&lights), screen);
    screen.export("./out/05out.png").expect("Failed to save image.");
}
