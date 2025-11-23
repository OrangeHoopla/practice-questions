use std::collections::HashMap;

use image::{GenericImageView, GrayImage, ImageBuffer, Luma, Rgb, open};
use imageproc::{contrast::{ThresholdType, otsu_level, threshold}, distance_transform::{Norm, distance_transform}, morphology::{Mask, dilate, grayscale_open}, region_labelling::connected_components};
use rand::Rng;

fn main() {
    println!("Starting");

    let img = open("image.png").expect("Failed to open image");

    // 2. Convert to a 8-bit luma (grayscale) image
    let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let _ = gray_img.save("gray.png");

    // find threshold level
    let _otsu = otsu_level(&gray_img);

    // apply threshold level
    let thres = threshold(&gray_img, 161, ThresholdType::BinaryInverted);
    let _ = thres.save("threshold.png");

    // apply open morph
    let morph = grayscale_open(&thres, &Mask::square(3));
    let _ = morph.save("morph.png");

    // sure ground
    let sure = dilate(&morph, Norm::L2, 3);
    let _ = sure.save("sure.png");

    // apply dilate
    let distance = distance_transform(&morph, Norm::L2, imageproc::distance_transform::DistanceFrom::Background);
    let _ = distance.save("distance.png");

    let max = distance.iter().fold(std::u8::MIN, |a,b| a.max(*b));
    // foreground
    let foreground = threshold(&distance, max/2, ThresholdType::Binary);
    let _ = foreground.save("foreground.png");

    let unknown = diff(&sure, &foreground);
    let _ = unknown.save("unknown.png");
    

    let background_color = Luma([155u8]);
    let connected = connected_components(&foreground, imageproc::region_labelling::Connectivity::Eight, background_color);


    let mut rgb_image = ImageBuffer::new(connected.width(), connected.height());
    let mut color_map: HashMap<u32, [u8; 3]> = HashMap::new();
    color_map.insert(1, [0,0,0]);
    let mut rng = rand::rng();

    for i in 2..30 { // random map of colors
            color_map.insert(i, [rng.random_range(1..=254),rng.random_range(1..=254),rng.random_range(1..=254)]);
        }

    // Iterate through Luma pixels and convert to RGB
    for (x, y, pixel) in connected.enumerate_pixels() {
        let luma_value = pixel[0]; // Luma pixels have one channel
        // Create an Rgb pixel where R, G, and B are all the luma value
        let rgb_pixel: Rgb<u8> = Rgb(*color_map.get(&pixel[0]).unwrap());

        // Set the pixel in the new RGB image
        rgb_image.put_pixel(x, y, rgb_pixel);
    }
    
    let _ = rgb_image.save("connected.png");



    //https://www.geeksforgeeks.org/computer-vision/image-segmentation-with-watershed-algorithm-opencv-python/
    
}

fn diff(minuend: &GrayImage, subtrahend: &GrayImage) -> GrayImage {
    let mut out = minuend.clone();
    
    for (x, y, pixel) in subtrahend.enumerate_pixels() {

        let luma_value = pixel[0]; // Luma pixels have one channel
        if(luma_value > 0) {
            let minuend_pix = minuend.get_pixel(x, y);

            let new_pixel = minuend_pix[0] - luma_value;


            out.put_pixel(x, y, Luma([new_pixel]));
        }

    }

    out
}
