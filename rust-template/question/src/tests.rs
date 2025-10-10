
//TODO place rayon image code here
#[cfg(test)]
mod tests {
    use crate::sqrt;

    #[test]
    fn this_test_will_pass() {
        let value = sqrt(9.0);
        assert_eq!(value.unwrap(), 3.0);
    }

    #[test]
    fn this_test_will_fail() {
        let value = sqrt(9.0);
        assert_eq!(value.unwrap(), 5.0);
    }
}
//https://geo-ant.github.io/blog/2022/implementing-parallel-iterators-rayon/

struct ParDataIter<'a> {
  data_slice : &'a [Data]
}

impl<'a> ParallelIterator for ParDataIter<'a> {
    type Item = &'a Data;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item> {
        bridge(self,consumer)
    }

    fn opt_len(&self) -> Option<usize> {
      Some(self.len())
    }
}

impl<'a> IndexedParallelIterator for ParDataIter<'a> {
    fn with_producer<CB: ProducerCallback<Self::Item>>(
        self,
        callback: CB,
    ) -> CB::Output {
        todo!()
    }

    fn drive<C: Consumer<Self::Item>>(self, consumer: C) -> C::Result {
        bridge(self,consumer)
    }

    fn len(&self) -> usize {
        self.data_slice.len()
    }
}

struct DataProducer<'a> {
  data_slice : &'a [Data],
}

