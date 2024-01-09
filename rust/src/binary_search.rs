use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet(name = "binary_search")]
pub mod binary_search {
    use std::{cmp::Ordering, collections::VecDeque};
    pub trait BinarySearch<T> {
        fn lower_bound(&self, x: &T) -> Option<usize>;
        fn upper_bound(&self, x: &T) -> Option<usize>;
    }
    macro_rules! impl_binary_search {
        ($($t:ty)*) => ($(
            impl<T: Ord> BinarySearch<T> for $t {
                fn lower_bound(&self, x: &T) -> Option<usize> {
                    let mut left = 0;
                    let mut right = self.len();
                    let mut result = None;
                    while left < right {
                        let med = (left + right) / 2;
                        match self[med].cmp(x) {
                            Ordering::Less => {
                                left = med + 1;
                            }
                            Ordering::Equal | Ordering::Greater => {
                                result = Some(med);
                                right = med;
                            }
                        }
                    }
                    result
                }
                fn upper_bound(&self, x: &T) -> Option<usize> {
                    let mut left = 0;
                    let mut right = self.len();
                    let mut result = None;
                    while left < right {
                        let med = (left + right) / 2;
                        match self[med].cmp(x) {
                            Ordering::Equal | Ordering::Less => {
                                left = med + 1;
                            }
                            Ordering::Greater => {
                                result = Some(med);
                                right = med;
                            }
                        }
                    }
                    result
                }
            }
        )*)
    }
    impl_binary_search!([T] VecDeque<T>);
}

#[cfg(test)]
mod tests {
    use super::binary_search::*;
    use rand::{thread_rng, Rng};
    #[test]
    fn test_binary_search() {
        let vec = [-100, 0, 1, 2, 10, 100, 1000];
        assert_eq!(vec.lower_bound(&1), Some(2));
        assert_eq!(vec.lower_bound(&100), Some(5));
        assert_eq!(vec.lower_bound(&-5000), Some(0));
        assert_eq!(vec.lower_bound(&999), Some(6));
        assert_eq!(vec.lower_bound(&1001), None);

        assert_eq!(vec.upper_bound(&-101), Some(0));
        assert_eq!(vec.upper_bound(&0), Some(2));
        assert_eq!(vec.upper_bound(&100), Some(6));
        assert_eq!(vec.upper_bound(&1000), None);
    }

    #[test]
    fn test_random_binary_search() {
        let test_case = 50;
        let max_value = 200;
        let query_num = 100;
        let mut rng = thread_rng();
        for _ in 0..test_case {
            let mut seq: Vec<i32> = (0..500).map(|_| rng.gen_range(0, max_value)).collect();
            seq.sort_unstable();

            for _ in 0..query_num {
                let query = rng.gen_range(0, max_value + 100);
                let result = (seq.lower_bound(&query), seq.upper_bound(&query));
                //sequential search
                let naive_search = |x: i32| {
                    let mut lower_result = None;
                    let mut upper_result = None;
                    for (i, &v) in seq.iter().enumerate() {
                        if lower_result.is_none() && x <= v {
                            lower_result = Some(i);
                        }
                        if upper_result.is_none() && x < v {
                            upper_result = Some(i);
                        }
                    }
                    (lower_result, upper_result)
                };

                assert_eq!(result, naive_search(query));
            }
        }
    }
}
