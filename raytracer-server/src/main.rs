use raytracer::entity::Entity;
use raytracer::entity::plane::Plane;
use raytracer::entity::scene::Scene;
use raytracer::entity::sphere::Sphere;
use raytracer::light::Light;
use raytracer::material::Material;
use raytracer::render::RenderState;
use raytracer::utils::{MaterialBuf, VecBuf};
use raytracer::vec3::Vec3;

use raytracer_server::service::RaytracerService;
use raytracer_server::grpc::renderer_server::RendererServer;

use tonic::transport::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = RenderState {
        width: 400,
        height: 300,
        fov: 0.95,
        origin: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        view_dir: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        interest_point: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -17.0,
        },
        vec_buf: VecBuf { points: vec![] },
        material_buf: MaterialBuf { materials: vec![] },
        scene: Scene::new(vec![].into()),
        background_color: Vec3 {
            x: 0.2,
            y: 0.7,
            z: 0.8,
        },
        recursion_limit: 7,
        lights: vec![],
    };

    let ivory = Material::new(1.0, [0.6, 0.3, 0.1, 0.0], Vec3::new(0.4, 0.4, 0.3), 50.0);
    let _ivory_idx = state.push_material(ivory);

    let glass = Material::new(1.5, [0.0, 0.5, 0.1, 0.8], Vec3::new(0.6, 0.7, 0.8), 125.0);
    // let glass_idx = state.push_material(glass);

    let red_rubber = Material::new(1.0, [0.9, 0.1, 0.0, 0.0], Vec3::new(0.3, 0.1, 0.1), 10.0);
    let _red_rubber_idx = state.push_material(red_rubber);

    let mirror = Material::new(1.0, [0.0, 10.0, 0.8, 0.0], Vec3::new(1.0, 1.0, 1.0), 1425.0);
    // let mirror_idx = state.push_material(mirror);

    let blue_rubber = Material::new(1.0, [0.9, 0.1, 0.0, 0.0], Vec3::new(0.1, 0.1, 0.3), 10.0);
    // let blue_rubber_idx = state.push_material(blue_rubber);

    let spheres = [
        Sphere::new(Vec3::new(-3.0, 0.0, -16.0), 2.0, ivory),
        Sphere::new(Vec3::new(-1.0, -1.5, -12.0), 2.0, glass),
        Sphere::new(Vec3::new(1.5, -0.5, -18.0), 3.0, red_rubber),
        Sphere::new(Vec3::new(7.0, 5.0, -18.0), 4.0, mirror),
    ];

    let planes = [
        Plane::new(Vec3::new(0.0, -4.0, 0.0), Vec3::new(0.0, 1.0, 0.0), ivory),
        Plane::new(
            Vec3::new(0.0, 60.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            red_rubber,
        ),
        Plane::new(Vec3::new(0.0, 0.0, -60.0), Vec3::new(0.0, 0.0, 1.0), blue_rubber),
        Plane::new(Vec3::new(0.0, 0.0, 60.0), Vec3::new(0.0, 0.0, -1.0), mirror),
        Plane::new(Vec3::new(35.0, 0.0, 0.0), Vec3::new(-1.0, 0.0, 0.0), red_rubber),
        Plane::new(Vec3::new(-35.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), mirror),
    ];

    state.lights = [
        Light::new(Vec3::new(-20.0, 20.0, 20.0), 1.5),
        Light::new(Vec3::new(30.0, 50.0, -25.0), 1.8),
        Light::new(Vec3::new(30.0, 20.0, 30.0), 1.7),
        Light::new(Vec3::new(-20.0, -20.0, -30.0), 10.0),
    ].into();

    state.scene = Scene::new(
        spheres
            .into_iter()
            .map(Entity::Sphere)
            .chain(planes.into_iter().map(Entity::Plane))
            .collect::<Vec<Entity>>(),
    );

    println!("Default state:");
    println!("Resolution: {}x{}", state.width, state.height);
    println!("Recursion depth: {}", state.recursion_limit);
    println!("Points: {}", state.vec_buf.points.len());
    println!("Lights: {}", state.lights.len());
    println!("Materials: {}", state.material_buf.materials.len());

    let raytracer_server = RaytracerService::new(state);
    let addr = "127.0.0.1:4242".parse().unwrap();

    Server::builder()
        .add_service(RendererServer::new(raytracer_server))
        .serve(addr)
        .await?;

    Ok(())
}