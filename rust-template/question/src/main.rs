use std::time::Instant;

use image::{DynamicImage, ImageBuffer, Luma, Rgb};
use rayon::{
    iter::{
        plumbing::{bridge, Consumer, Producer, ProducerCallback, UnindexedConsumer},
        IndexedParallelIterator, IntoParallelIterator, IntoParallelRefIterator, ParallelIterator,
    },
    slice::{ParallelSlice, ParallelSliceMut},
};

pub mod tests;
// not to self start writing code for watershed algorthym 
// https://docs.opencv.org/4.x/d3/db4/tutorial_py_watershed.html
fn main() {
    let mut _data = DataCollection {
        data: vec![1, 2, 3, 4],
    };
    // println!("Quade");

    // // println!("data = {:?}", data);

    // let sum_of_squares: Data = data.par_iter().map(|x| x * x).sum();

    // println!("sum = {}", sum_of_squares);

    let mut img: ImageBuffer<Luma<u16>, Vec<u16>> = ImageBuffer::new(4000, 4000);
    img.put_pixel(0, 0, Luma([150]));

    let mut iter = img.chunks_mut(7);
    let a = iter.next().unwrap();
    let mut b = iter.next().unwrap();
    for i in b {
        *i = 12 as u16;
    }
    

    // print!("{:?} ", a);
    // println!("{:?} ", b);
    let start = Instant::now();
    let mut res = img.par_chunks_exact_mut(1)
    .for_each(| x: &mut [u16]| { x[0] = x[0] + 2; });
    let duration = start.elapsed();

    let test = img.get_pixel(12, 12);
    println!("{:?}", test);
    println!("Test 'my_timed_test' took: {:?}", duration);
}

pub fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
    // maybe include the template here
}

type Data = i32;

struct DataCollection {
    data: Vec<Data>,
}
struct ParDataIter<'a> {
    data_slice: &'a [Data],
}

impl<'a> ParallelIterator for ParDataIter<'a> {
    type Item = &'a Data;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge(self, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'a> IndexedParallelIterator for ParDataIter<'a> {
    fn with_producer<CB: ProducerCallback<Self::Item>>(self, callback: CB) -> CB::Output {
        let producer = DataProducer::from(self);
        callback.callback(producer)
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        self.data_slice.len()
    }
}

struct DataProducer<'a> {
    data_slice: &'a [Data],
}

impl<'a> Producer for DataProducer<'a> {
    type Item = &'a Data;
    type IntoIter = std::slice::Iter<'a, Data>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_slice.iter()
    }

    fn split_at(self, index: usize) -> (Self, Self) {
        let (left, right) = self.data_slice.split_at(index);
        (
            DataProducer { data_slice: left },
            DataProducer { data_slice: right },
        )
    }
}

impl<'a> From<ParDataIter<'a>> for DataProducer<'a> {
    fn from(iterator: ParDataIter<'a>) -> Self {
        Self {
            data_slice: iterator.data_slice,
        }
    }
}

// impl<'a> IndexedParallelIterator for ParDataIter<'a> {
//     fn with_producer<CB: ProducerCallback<Self::Item>>(
//         self,
//         callback: CB,
//     ) -> CB::Output {
//         let producer = DataProducer::from(self);
//         callback.callback(producer)
//     }

//     fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
//         bridge(self,consumer)
//     }

//     fn len(&self) -> usize {
//         self.data_slice.len()
//     }
// }

impl DataCollection {
    pub fn parallel_iterator(&self) -> ParDataIter {
        ParDataIter {
            data_slice: &self.data,
        }
    }
}

impl<'a> IntoParallelIterator for &'a DataCollection {
    type Iter = ParDataIter<'a>;
    type Item = &'a Data;

    fn into_par_iter(self) -> Self::Iter {
        ParDataIter {
            data_slice: &self.data,
        }
    }
}
