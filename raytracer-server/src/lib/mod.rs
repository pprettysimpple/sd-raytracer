pub mod operation;
pub mod service;
mod render;
pub mod default_scene;

pub mod grpc {
    tonic::include_proto!("raytracer_proto");
}

#[cfg(test)]
mod test {
    use std::io;
    use std::thread::sleep;
    use std::time::Duration;
    use image::EncodableLayout;
    use log::{info, warn};
    use tonic::transport::Server;
    use crate::default_scene::create_default_state;
    use crate::grpc::renderer_client::RendererClient;
    use crate::grpc::renderer_server::RendererServer;
    use crate::grpc::RenderRequest;
    use crate::service::RaytracerService;

    #[tokio::test]
    async fn fat_test_client_server() -> io::Result<()> {
        let conn_string = "127.0.0.1:4242";
        let conn_string_with_schema = "http://".to_owned() + conn_string;

        let j_handle = tokio::spawn(async {
            let _server = Server::builder()
                .add_service(RendererServer::new(RaytracerService::new(create_default_state())))
                .serve(conn_string.parse().unwrap())
                .await
                .expect("Server broken");
        });

        while let Err(err) = RendererClient::connect(conn_string_with_schema.clone()).await {
            warn!("Failed to connect to server {err}");
            sleep(Duration::from_secs(1));
        }

        let mut client = RendererClient::connect(conn_string_with_schema)
            .await
            .expect("Failed to connect with gRPC server");

        let mut resp = client
            .render(RenderRequest { operations: vec![] })
            .await
            .expect("Failed to call gPRC procedure")
            .into_inner();

        let mut buf = Vec::new();
        while let Some(mut chunk) = resp.message().await.expect("Failed to fetch chunk") {
            buf.append(chunk.picture_data.as_mut());
        }

        info!("Result buffer size: {}", buf.len());

        let actual = image::ImageBuffer::from(image::RgbImage::from_raw(100, 50, buf)
            .expect("Failed: Broken picture bytes")
        );

        let expected = image::io::Reader::open("canondata/result.png")
            .expect("Failed to fetch canondata at canondata/result.png")
            .decode()
            .expect("Failed to parse canondata");

        assert_eq!(expected.as_bytes(), actual.as_bytes());

        if j_handle.is_finished() { // so, there is an error
            j_handle.await.expect("Server panic")
        }

        Ok(())
    }
}