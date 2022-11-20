use std::sync::Mutex;
use log::info;

use crate::grpc::renderer_server::Renderer;
use crate::grpc::{RenderRequest, RenderResponse};
use crate::operation::RenderStateOperation;
use crate::render::render_picture;

use raytracer::render::RenderState;

use tokio::sync::mpsc;

use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct RaytracerService {
    render_state: Mutex<RenderState>,
}

impl RaytracerService {
    pub fn new(render_state: RenderState) -> RaytracerService {
        RaytracerService {
            render_state: Mutex::new(render_state),
        }
    }
}

#[tonic::async_trait]
impl Renderer for RaytracerService {
    type RenderStream = ReceiverStream<Result<RenderResponse, Status>>;

    async fn render(&self, request: Request<RenderRequest>) -> Result<Response<Self::RenderStream>, Status> {
        let mut state_copy = self.render_state.lock().unwrap().clone();
        let (tx, rx) = mpsc::channel(16);

        info!("Apply operations");
        for op in request.into_inner().operations {
            if let Some(op) = op.operation {
                op.apply_to_render_state(&mut state_copy);
            }
        }

        let mut state = self.render_state
            .lock()
            .expect("Failed to lock state");
        *state = state_copy.clone();

        info!("State updated");

        tokio::spawn(async move {
            info!("Render picture");
            if let Some(buffer) = render_picture(&state_copy) {
                info!("Ok");
                for chunk in buffer.chunks(1024 * 16).into_iter() {
                    info!("Send chunk of size {}", chunk.len());
                    tx.send(Ok(RenderResponse {
                        picture_data: Vec::from(chunk),
                    })).await.unwrap()
                }
            } else {
                info!("Err rendering");
                tx.send(
                    Err(Status::aborted("Failed to render picture"))
                ).await.unwrap()
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use raytracer::entity::scene::Scene;
    use raytracer::render::RenderState;
    use raytracer::utils::{MaterialBuf, VecBuf};
    use raytracer::vec3::Vec3;
    use tonic::Request;
    use crate::grpc;
    use crate::grpc::renderer_server::Renderer;
    use crate::grpc::{Operation, Origin, RenderRequest};
    use crate::grpc::operation::Operation::SetOrigin;
    use crate::service::RaytracerService;

    fn create_default_service() -> RaytracerService {
        let state = RenderState {
            width: 10,
            height: 10,
            fov: 1.0,
            origin: Default::default(),
            view_dir: Vec3::new(0.0, 0.0, 1.0),
            background_color: Default::default(),
            recursion_limit: 1,
            interest_point: Default::default(),
            vec_buf: VecBuf { points: vec![] },
            material_buf: MaterialBuf { materials: vec![] },
            scene: Scene { entities: vec![] },
            lights: vec![],
        };

        RaytracerService::new(state)
    }

    #[tokio::test]
    async fn test_render_no_op_black() -> io::Result<()> {
        let service = create_default_service();

        let request = Request::new(RenderRequest { operations: vec![] });

        let mut rcv = service.render(request)
            .await
            .expect("Failed to rcv result")
            .into_inner()
            .into_inner();

        while let Some(part) = rcv.recv().await {
            assert!(part.is_ok());
            assert!(part.unwrap().picture_data.iter().all(|val| *val == 0_u8));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_render_no_op_colored() -> io::Result<()> {
        let service = create_default_service();

        service.render_state
            .lock()
            .unwrap()
            .background_color = Vec3::new(0.1, 0.1, 0.1);

        let request = Request::new(RenderRequest { operations: vec![] });

        let mut rcv = service.render(request)
            .await
            .expect("Failed to rcv result")
            .into_inner()
            .into_inner();

        while let Some(part) = rcv.recv().await {
            assert!(part.is_ok());
            assert!(part.unwrap().picture_data.iter().all(|val| *val == 25_u8));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_render_simple_op() -> io::Result<()> {
        let service = create_default_service();

        let request = Request::new(RenderRequest {
            operations: vec![Operation {
                operation: Option::from(SetOrigin(Origin {
                    origin: Option::from(grpc::Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    })
                }))
            }]
        });

        assert_eq!(service.render_state.lock().unwrap().origin, Default::default());

        let mut rcv = service.render(request)
            .await
            .expect("Failed to rcv result")
            .into_inner()
            .into_inner();

        while let Some(part) = rcv.recv().await {
            assert!(part.is_ok());
        }

        assert_eq!(service.render_state.lock().unwrap().origin, Vec3::new(1.0, 1.0, 1.0));

        Ok(())
    }
}

