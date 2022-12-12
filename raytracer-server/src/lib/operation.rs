use raytracer::render::RenderState;
use raytracer::vec3::Vec3;
use crate::grpc::{Fov, Origin, Resolution, ViewDirection};
use crate::grpc::operation::Operation;

pub trait RenderStateOperation {
    fn apply_to_render_state(&self, state: &mut RenderState);
}

impl RenderStateOperation for Operation {
    fn apply_to_render_state(&self, state: &mut RenderState) {
        match self {
            Operation::SetFov(Fov { fov }) => state.fov = *fov as f32,
            Operation::SetResolution(Resolution { width, height }) => {
                state.width = *width as usize;
                state.height = *height as usize;
            }
            Operation::SetViewDirection(ViewDirection { direction }) => {
                if direction.is_some() {
                    let dir = direction.clone().unwrap();
                    state.view_dir = Vec3 {
                        x: dir.x as f32,
                        y: dir.y as f32,
                        z: dir.z as f32,
                    }
                }
            }
            Operation::SetOrigin(Origin { origin }) => {
                if origin.is_some() {
                    let origin = origin.clone().unwrap();
                    state.origin = Vec3 {
                        x: origin.x as f32,
                        y: origin.y as f32,
                        z: origin.z as f32,
                    }
                }
            }
        }
    }
}