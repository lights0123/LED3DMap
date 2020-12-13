use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use image::imageops::grayscale;
use image::{GenericImageView, ImageBuffer, Luma, Pixel};
use imageproc::definitions::Image;
use imageproc::filter::gaussian_blur_f32;
use num_traits::SaturatingSub;

fn subtract<P, Container>(lhs: &mut ImageBuffer<P, Container>, rhs: &ImageBuffer<P, Container>)
where
    P: Pixel + 'static,
    P::Subpixel: SaturatingSub + Copy + Debug + 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    lhs.pixels_mut().zip(rhs.pixels()).for_each(|(l, r)| {
        l.channels_mut()
            .iter_mut()
            .zip(r.channels().iter())
            .for_each(|(l, r)| {
                *l = l.saturating_sub(r);
            })
    });
}

// https://github.com/atomic14/self-organising-leds/blob/40ac412504d0867dfde33adfe9c2e268f5b19a3b/frontend/src/imageProcessing/imageProcessing.tsx#L145
fn find_blob<P, Container>(lit: &ImageBuffer<P, Container>) -> Option<(u64, u64, u8)>
where
    P: Pixel<Subpixel = u8> + 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
    let max_difference = lit.pixels().map(|px| px.channels()[0]).max().unwrap() / 2;
    let (x_pos, y_pos, total) = lit
        .rows()
        .enumerate()
        .flat_map(|(y, row)| row.enumerate().map(move |(x, px)| (x, y, px.channels()[0])))
        .fold((0, 0, 0), |(x_pos, y_pos, total), (x, y, diff)| {
            if diff > max_difference {
                (
                    x_pos + x as u64 * diff as u64,
                    y_pos + y as u64 * diff as u64,
                    total + diff as u64,
                )
            } else {
                (x_pos, y_pos, total)
            }
        });
    if total == 0 {
        return None;
    }
    Some((x_pos / total, y_pos / total, max_difference))
}

pub fn read_base_frame<P: Pixel<Subpixel = u8>, I: GenericImageView<Pixel = P>>(
    base: &I,
) -> Image<Luma<u8>> {
    let base = grayscale(base);
    gaussian_blur_f32(&base, 3.)
}

pub fn compute_light_pos<P: Pixel<Subpixel = u8>, I: GenericImageView<Pixel = P>>(
    base: &Image<Luma<u8>>,
    lit: &I,
) -> Option<(u64, u64, u8)> {
    let lit = grayscale(lit);
    let mut lit = gaussian_blur_f32(&lit, 3.);
    subtract(&mut lit, &base);
    find_blob(&lit)
}
