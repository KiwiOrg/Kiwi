use ambient_app::{App, AppBuilder};
use ambient_core::{
    camera::active_camera,
    main_scene,
    transform::{scale, translation},
};
use ambient_element::ElementComponentExt;
use ambient_primitives::{Cube, Quad};
use ambient_renderer::{cast_shadows, color, outline};
use ambient_std::math::SphericalCoords;
use glam::{uvec2, vec3, vec4, Vec3, Vec4};
use std::{process::exit, time::Duration};

async fn init(app: &mut App) {
    let world = &mut app.world;

    Cube.el()
        .set(color(), vec4(0.5, 0.5, 0.5, 1.))
        .set(translation(), Vec3::Z)
        .set_default(cast_shadows())
        .set(outline(), Vec4::ONE)
        .spawn_static(world);
    Quad.el().set(scale(), Vec3::ONE * 10.).spawn_static(world);

    ambient_cameras::spherical::new(vec3(0., 0., 0.), SphericalCoords::new(std::f32::consts::PI / 4., std::f32::consts::PI / 4., 5.))
        .set(active_camera(), 0.)
        .set(main_scene(), ())
        .spawn(world);
    tokio::time::sleep(Duration::from_secs_f32(1.)).await;
    exit(0);
}

fn main() {
    // wgpu_subscriber::initialize_default_subscriber(None);
    AppBuilder::simple().headless(Some(uvec2(400, 400))).block_on(init);
}
