use std::time::UNIX_EPOCH;
use std::{collections::BinaryHeap, time::SystemTime};
use std::cmp::Ordering;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb, open};
use imageproc::{
    contrast::{ThresholdType, otsu_level, threshold},
    distance_transform::{Norm, distance_transform},
    morphology::{Mask, dilate, grayscale_open},
    region_labelling::connected_components,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Pixel {
    cost: u8,
    position: (u32,u32),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for Pixel {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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
    let distance = distance_transform(
        &morph,
        Norm::L2,
        imageproc::distance_transform::DistanceFrom::Background,
    );
    let _ = distance.save("distance.png");

    let max = distance.iter().fold(std::u8::MIN, |a, b| a.max(*b));
    // foreground
    let foreground = threshold(&distance, max / 2, ThresholdType::Binary);
    let _ = foreground.save("foreground.png");

    let unknown = diff(&sure, &foreground);
    let _ = unknown.save("unknown.png");


    let background_color = Luma([155u8]);
    let connected = connected_components(
        &foreground,
        imageproc::region_labelling::Connectivity::Eight,
        background_color,
    );

    let connected_unknown = diff_zero(&connected, &unknown);
    let _ = connected_unknown.save("connected_unknown.png"); // this is the mask to be feed into the watershed
    
    // println!("{:?}", connected_unknown.get_pixel(0, 0));

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let answer = watershed(img, connected_unknown); //main
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let total = end-start;
    println!("{:?}",total);

    let _ = answer.save("watershed.png");

    //https://www.geeksforgeeks.org/computer-vision/image-segmentation-with-watershed-algorithm-opencv-python/
    //https://github.com/opencv/opencv/blob/b1d75bf477e77373b420d31ddf36709c0907dd32/modules/imgproc/src/segmentation.cpp#L88
}

fn diff(minuend: &GrayImage, subtrahend: &GrayImage) -> GrayImage {
    let mut out = minuend.clone();

    for (x, y, pixel) in subtrahend.enumerate_pixels() {
        let luma_value = pixel[0]; // Luma pixels have one channel
        if luma_value > 0 {
            let minuend_pix = minuend.get_pixel(x, y);

            let new_pixel = minuend_pix[0] - luma_value;

            out.put_pixel(x, y, Luma([new_pixel]));
        }
    }

    out
}

/**
 * Sets any pixel on minuend to zero for pixels that are above zero for subtrahend
 */
fn diff_zero(minuend: &ImageBuffer<Luma<u32>, Vec<u32>>, subtrahend: &GrayImage) -> GrayImage {
    let mut out: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(minuend.width(), minuend.height());

    for (x, y, pixel) in minuend.enumerate_pixels() {
        let sub = subtrahend.get_pixel(x, y)[0];

        if sub > 0 {
            out.put_pixel(x, y, Luma([0]));
        } else {
            out.put_pixel(x, y, Luma([pixel[0].try_into().unwrap()]));
        }
    }

    out
}


/**
 * The background is a group(1) the unknown is a blur around the part we are fighting over
 * anything that is 0 is considered unknown
 * we use the diff in the actual image contrast to determine what group it goes to
 * 
 */
fn watershed(_src: DynamicImage, markers: ImageBuffer<Luma<u8>, Vec<u8>>) -> GrayImage {

    let mut priority_queue: BinaryHeap<Pixel> = BinaryHeap::new();
    // priority_queue.push(State { cost: 0, position: 0 });


    let mut copy: ImageBuffer<Luma<u8>, Vec<u8>> = markers.clone().try_into().unwrap();
    println!( "{:?}", markers.get_pixel(0, 0));
    let img = _src.to_rgb8();

    let in_queue: u8 = 254;
    let wshed: u8 = 255;

    // width perimeter
    for i in 0..copy.width() {
        copy.put_pixel(i, 0, Luma([wshed]));
        copy.put_pixel(i, copy.height()-1, Luma([wshed]));
    }

    // height perimeter


    // initial phase: put all the neighbor pixels of each marker to the ordered queue -
    // determine the initial boundaries of the basins
    for i in 1..copy.height()-1 {
        copy.put_pixel(0, i, Luma([wshed]));
        copy.put_pixel(copy.width()-1, i, Luma([wshed]));
        for j in 1..copy.width()-1 {

            // making sure nothing in queue
            if (copy.get_pixel(j, i).0[0] < (0 as u8)) || (copy.get_pixel(j, i).0[0] > 253) {
                copy.put_pixel(j, i,Luma([0]));
            }

            // checks surrounding pixels to see if around a group
            if  copy.get_pixel(j, i).0[0] == 0 && 
                (((copy.get_pixel(j, i+1).0[0] > 0) && (copy.get_pixel(j, i+1).0[0] < 254)) || 
                ((copy.get_pixel(j, i-1).0[0] > 0) && (copy.get_pixel(j, i-1).0[0] < 254)) || 
                ((copy.get_pixel(j+1, i).0[0] > 0) && (copy.get_pixel(j+1, i).0[0] < 254)) || 
                ((copy.get_pixel(j-1, i).0[0] > 0) && (copy.get_pixel(j-1, i).0[0] < 254))) 
            {
                // the lower the priority the sooner it gets addressed
                let mut priority: u8 = 255;
                let mut holder:u8 = 255;

                if (copy.get_pixel(j, i+1).0[0] > 0) && (copy.get_pixel(j, i+1).0[0] < 254) {
                    holder = pixel_diff(img.get_pixel(j, i+1).clone(), img.get_pixel(j, i).clone());
                    priority = std::cmp::min(holder, priority);
                }
                if (copy.get_pixel(j, i-1).0[0] > 0) && (copy.get_pixel(j, i-1).0[0] < 254) {

                    holder = pixel_diff(img.get_pixel(j, i-1).clone(), img.get_pixel(j, i).clone());
                    priority = std::cmp::min(holder, priority);
                    
                }
                if (copy.get_pixel(j+1, i).0[0] > 0) && (copy.get_pixel(j+1, i).0[0] < 254) {

                    holder = pixel_diff(img.get_pixel(j+1, i).clone(), img.get_pixel(j, i).clone());
                    priority = std::cmp::min(holder, priority);
                    
                }
                if (copy.get_pixel(j-1, i).0[0] > 0) && (copy.get_pixel(j-1, i).0[0] < 254) {

                    holder = pixel_diff(img.get_pixel(j-1, i).clone(), img.get_pixel(j, i).clone());
                    priority = std::cmp::min(holder, priority);
                    
                }
                // add to queue
                priority_queue.push(Pixel {cost: priority, position: (j,i)});
                copy.put_pixel(j, i, Luma([in_queue])); //3722 or 3627
                // need to designate pixel in queue
                

            }


        }
        
        

    }

    // next step
    // println!("{}", priority_queue.len());
    // let mut current: State = State { cost: 12, position: (0,0) };
    let mut i = 0;
    while !priority_queue.is_empty() {
        i += 1;
        let current = priority_queue.pop().unwrap();
        let mut lab = 0;
        let mut t;

        //left
        t = copy.get_pixel(current.position.0-1, current.position.1).0[0];
        if t > 0 && t < 254 {
            lab = t;
        }
        //right
        t = copy.get_pixel(current.position.0+1, current.position.1).0[0];
        if t > 0 && t < 254 {
            if lab == 0 {lab=t;}
            else if t != lab {lab=wshed;}
        }
        //top
        t = copy.get_pixel(current.position.0, current.position.1-1).0[0];
        if t > 0 && t < 254 {
            if lab == 0 {lab=t;}
            else if t != lab {lab=wshed;}
        }
        //bottom
        t = copy.get_pixel(current.position.0, current.position.1+1).0[0];
        if t > 0 && t < 254 {
            if lab == 0 {lab=t;}
            else if t != lab {lab=wshed;}
        }

        copy.put_pixel(current.position.0, current.position.1, Luma([lab]));

        // halfway
        if lab == wshed {continue;}

        //left
        if copy.get_pixel(current.position.0-1, current.position.1).0[0] == 0 {

            let holder = pixel_diff(img.get_pixel(current.position.0-1, current.position.1).clone(), 
                                img.get_pixel(current.position.0, current.position.1).clone());
            
            priority_queue.push(Pixel { cost: holder, position: (current.position.0-1, current.position.1) });
            copy.put_pixel(current.position.0-1, current.position.1, Luma([in_queue]));
        }
        //right
        if copy.get_pixel(current.position.0+1, current.position.1).0[0] == 0 {

            let holder = pixel_diff(img.get_pixel(current.position.0+1, current.position.1).clone(), 
                                img.get_pixel(current.position.0, current.position.1).clone());
            
            priority_queue.push(Pixel { cost: holder, position: (current.position.0+1, current.position.1) });
            copy.put_pixel(current.position.0+1, current.position.1, Luma([in_queue]));
        }
        //top
        if copy.get_pixel(current.position.0, current.position.1-1).0[0] == 0 {

            let holder = pixel_diff(img.get_pixel(current.position.0, current.position.1-1).clone(), 
                                img.get_pixel(current.position.0, current.position.1).clone());
            
            priority_queue.push(Pixel { cost: holder, position: (current.position.0, current.position.1-1) });
            copy.put_pixel(current.position.0, current.position.1-1, Luma([in_queue]));
        }
        //bottom
        if copy.get_pixel(current.position.0, current.position.1+1).0[0] == 0 {

            let holder = pixel_diff(img.get_pixel(current.position.0, current.position.1+1).clone(), 
                                img.get_pixel(current.position.0, current.position.1).clone());
            
            priority_queue.push(Pixel { cost: holder, position: (current.position.0, current.position.1+1) });

            copy.put_pixel(current.position.0, current.position.1+1, Luma([in_queue]));
        }
        if i % 30 == 0 {
            let _ = copy.save(format!("gif1/{}.png", i));
        }

    }

    // println!("{:?}", current);

    // println!("{:?}", priority_queue.len());
    
    // visualization tool
    // priority_queue.iter().for_each(|x| copy.put_pixel(x.position.0, x.position.1, Luma([255])));


    // setting



    copy
}


fn pixel_diff(a: Rgb<u8>, b: Rgb<u8>) -> u8{
    let db = a.0[0].abs_diff(b.0[0]);
    let dg = a.0[1].abs_diff(b.0[1]);
    let dr = a.0[2].abs_diff(b.0[2]);
    let diff = std::cmp::max(db, dg);
    
    std::cmp::max(diff, dr)


}

