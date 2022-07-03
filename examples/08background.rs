use raytracer::{camera::{Camera, Screen}, surface::Scene, meshes::{Plane, Sphere}, materials::{SDRMaterial}, light::{SceneLights, PointLight}, background::BackgroundSurface};

fn main() {
    let camera = Camera::new(
        (12.0, 12.0, 16.0).into(),
        (0.0, 0.0, 0.0).into(),
        10.0,
        (0.0, 1.0, 0.0).into()
    );

    let mut scene: Scene = Vec::new();
    let mut lights: SceneLights = Vec::new();

    scene.push(Box::new(Plane::new(
        Box::new(SDRMaterial::new([255, 255, 255].into(), 0.5, 0.1, 10.0, 0.0)),
        (0.0, -3.0, 0.0).into(),
        (0.0, 1.0, 0.0).into(),
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(SDRMaterial::new([255, 0, 0].into(), 1.0, 0.3, 5.0, 0.8)),
        (0.0, 0.0, 0.0).into(),
        3.0,
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(SDRMaterial::new([0, 255, 0].into(), 0.6, 0.2, 5.0, 0.3)),
        (8.0, 0.0, 0.0).into(),
        3.0,
    )));

    scene.push(Box::new(Sphere::new(
        Box::new(SDRMaterial::new([0, 0, 255].into(), 1.0, 0.1, 5.0, 0.1)),
        (-8.0, 0.0, 0.0).into(),
        3.0,
    )));

    scene.push(Box::new(BackgroundSurface::new("./res/background/colosseum_4k.png")));

    lights.push(Box::new(PointLight::new((14.0, 2.0, -2.0).into(), [255, 0, 255].into(), 70.0).with_falloff()));
    lights.push(Box::new(PointLight::new((-10.0, 2.0, -5.0).into(), [255, 255, 0].into(), 90.0).with_falloff()));
    lights.push(Box::new(PointLight::new((0.0, 2.0, 5.0).into(), [0, 255, 255].into(), 100.0).with_falloff()));

    //lights.push(Box::new(PointLight::new((0.0, 10.0, 4.0).into(), [255, 255, 255].into(), 50.0).with_falloff()));
    //lights.push(Box::new(DirectionalLight::new((0.0, 10.0, 4.0).into(), [255, 255, 255].into(), 5.0)));
    
    let mut screen = Screen::new(1920, 1080, 16.0, 9.0);

    screen = camera.render_scene(&scene, Some(&lights), screen);
    screen.export("./out/08out.png").expect("Failed to save image.");
}
