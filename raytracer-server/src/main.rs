use std::net::SocketAddr;

use argparse::{ArgumentParser, Store};
use log::info;

use raytracer_server::service::RaytracerService;
use raytracer_server::grpc::renderer_server::RendererServer;
use raytracer_server::default_scene::create_default_state;

use tonic::transport::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = create_default_state();

    info!("Default state:");
    info!("Resolution: {}x{}", state.width, state.height);
    info!("Recursion depth: {}", state.recursion_limit);
    info!("Points: {}", state.vec_buf.points.len());
    info!("Lights: {}", state.lights.len());
    info!("Materials: {}", state.material_buf.materials.len());

    let raytracer_server = RaytracerService::new(state);
    let mut addr = "127.0.0.1".to_string();
    let mut port = 4242;

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Run raytracer grpc service");
        parser.refer(&mut addr)
            .add_option(&["-a", "--address"], Store, "Address to run on");
        parser.refer(&mut port)
            .add_option(&["-p", "--port"], Store, "Port to run on");
        parser.parse_args_or_exit();
    }

    let mut addr: SocketAddr = (addr + ":" + port.to_string().as_str()).parse().unwrap();
    addr.set_port(port);

    Server::builder()
        .add_service(RendererServer::new(raytracer_server))
        .serve(addr)
        .await?;

    Ok(())
}