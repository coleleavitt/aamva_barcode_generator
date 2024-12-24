// use image::{imageops, DynamicImage, ImageBuffer, Luma};
// use crate::decode::try_decode_barcode;

// pub(crate) fn preprocess_image(img: DynamicImage) -> DynamicImage {
//     // Convert to grayscale first
//     let gray_img = img.grayscale();
//
//     // Enhance contrast for better barcode visibility
//     let contrast_factor = 2.0;
//     let gray_buffer = gray_img.to_luma8();
//     let contrast_img = imageops::contrast(&gray_buffer, contrast_factor);
//
//     // Apply binary threshold
//     let threshold_value = 128u8;
//     let mut binary_img = ImageBuffer::new(contrast_img.width(), contrast_img.height());
//     for (x, y, pixel) in contrast_img.enumerate_pixels() {
//         let new_value = if pixel.0[0] > threshold_value {
//             255u8
//         } else {
//             0u8
//         };
//         binary_img.put_pixel(x, y, Luma([new_value]));
//     }
//
//     let binary_img = DynamicImage::ImageLuma8(binary_img);
//
//     // First try original orientation
//     if try_decode_barcode(&binary_img).is_ok() {
//         return binary_img;
//     }
//
//     // // Try 90-degree rotations first
//     // let rotations = [
//     //     |img: &DynamicImage| img.clone(),
//     //     |img: &DynamicImage| DynamicImage::ImageLuma8(imageops::rotate90(&img.to_luma8())),
//     //     |img: &DynamicImage| DynamicImage::ImageLuma8(imageops::rotate180(&img.to_luma8())),
//     //     |img: &DynamicImage| DynamicImage::ImageLuma8(imageops::rotate270(&img.to_luma8())),
//     // ];
//
//     // // Try each rotation with flips
//     // for rotation in rotations.iter() {
//     //     let rotated = rotation(&binary_img);
//     //     if try_decode_barcode(&rotated).is_ok() {
//     //         return rotated;
//     //     }
//     //
//     //     // Try with horizontal flip
//     //     let flipped = DynamicImage::ImageLuma8(imageops::flip_horizontal(&rotated.to_luma8()));
//     //     if try_decode_barcode(&flipped).is_ok() {
//     //         return flipped;
//     //     }
//     //
//     //     // Try with vertical flip
//     //     let flipped = DynamicImage::ImageLuma8(imageops::flip_vertical(&rotated.to_luma8()));
//     //     if try_decode_barcode(&flipped).is_ok() {
//     //         return flipped;
//     //     }
//     // }
//
//     // Return original if no rotation worked
//     binary_img
// }