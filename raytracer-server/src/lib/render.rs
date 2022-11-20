use image::{ImageBuffer, Rgb, RgbImage};

use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::IndexedParallelIterator;
use rayon::iter::ParallelIterator;

use raytracer::render::RenderState;
use raytracer::vec3::Vec3;

fn show_frame_buffer(
    config: &RenderState,
    frame_buffer: &[Vec3],
) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let to_color = |color: f32| (color.min(1.0).max(0.0) * 255.0) as u8;
    RgbImage::from_vec(
        config.width as u32,
        config.height as u32,
        frame_buffer
            .iter()
            .map(|v| [v.x, v.y, v.z].map(to_color))
            .collect::<Vec<[u8; 3]>>()
            .concat(),
    )
}

pub(crate) fn render_picture(state: &RenderState) -> Option<Vec<u8>> {
    let len = state.height * state.width;
    let mut frame_buffer: Vec<Vec3> = vec![Default::default(); len];

    frame_buffer
        .par_iter_mut()
        .enumerate()
        .for_each(|(pix, vec)| {
            *vec = state.render_scene_pixel(pix)
        });

    show_frame_buffer(state, frame_buffer.as_ref())
        .map(|img| img.into_raw())
}