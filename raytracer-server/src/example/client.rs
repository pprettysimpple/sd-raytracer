use raytracer_server::grpc::renderer_client::RendererClient;
use raytracer_server::grpc::{Operation, RenderRequest, Resolution};
use raytracer_server::grpc::operation::Operation::SetResolution;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RendererClient::connect("http://127.0.0.1:4242")
        .await
        .expect("Failed to connect with gRPC server");

    let resolution = Resolution {
        width: 1000,
        height: 500,
    };

    let mut resp = client
        .render(RenderRequest {
            operations: vec![Operation {
                operation: Option::from(SetResolution(resolution.clone()))
            }]
        })
        .await
        .expect("Failed to call gPRC procedure")
        .into_inner();

    let mut buf = Vec::new();
    while let Some(mut chunk) = resp.message().await? {
        buf.append(chunk.picture_data.as_mut());
    }

    let img = image::ImageBuffer::from(image::RgbImage::from_raw(resolution.width, resolution.height, buf)
        .expect("Failed: Broken picture bytes")
    );

    img.save("result.png")
        .expect("Failed to save picture on disk");

    Ok(())
}
