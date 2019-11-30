use std::cmp::Ordering;

pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> Option<usize>;
    fn upper_bound(&self, x: &T) -> Option<usize>;
}

impl<T: Ord> BinarySearch<T> for [T] {
    //greater than or equal
    fn lower_bound(&self, x: &T) -> Option<usize> {
        let mut left = 0;
        let mut right = self.len();
        let mut result = None;
        while left < right {
            let mut med = (low + right) / 2;
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
            let mut med = (low + right) / 2;
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_binary_search() {
        let vec = vec![-100, 0, 1, 2, 10, 100, 1000];
        assert_eq!(vec.lower_bound(&1), Some(2));
    }
}
