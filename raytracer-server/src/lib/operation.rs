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
            Operation::SetFov(Fov { fov }) => state.fov = *fov,
            Operation::SetResolution(Resolution { width, height }) => {
                state.width = *width as usize;
                state.height = *height as usize;
            }
            Operation::SetViewDirection(ViewDirection { direction }) => {
                if direction.is_some() {
                    let dir = direction.clone().unwrap();
                    state.view_dir = Vec3 {
                        x: dir.x,
                        y: dir.y,
                        z: dir.z,
                    }
                }
            }
            Operation::SetOrigin(Origin { origin }) => {
                if origin.is_some() {
                    let origin = origin.clone().unwrap();
                    state.origin = Vec3 {
                        x: origin.x,
                        y: origin.y,
                        z: origin.z,
                    }
                }
            }
        }
    }
}