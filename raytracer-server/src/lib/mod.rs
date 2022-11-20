pub mod operation;
pub mod service;
mod render;

pub mod grpc {
    tonic::include_proto!("raytracer_proto");
}