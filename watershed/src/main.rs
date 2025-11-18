use image::{ImageBuffer, Luma, open};
use imageproc::{contrast::{ThresholdType, otsu_level, threshold}, gray_image};

fn main() {
    println!("Hello, world!");

    let image = gray_image!(
    10, 80, 20;
    50, 90, 70);

    let img = open("image.png").expect("Failed to open image");

    // 2. Convert to a 8-bit luma (grayscale) image
    let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = img.to_luma8();
    let _ = gray_img.save("gray.png");

    let result = threshold(&gray_img, 161, ThresholdType::BinaryInverted);
    let _ = result.save("threshold.png");

    //https://www.geeksforgeeks.org/computer-vision/image-segmentation-with-watershed-algorithm-opencv-python/
    
}
