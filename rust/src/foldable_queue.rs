use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
/// Sliding Window Aggregation
pub mod foldable_queue {
    use std::fmt::Debug;

    /// A data structure is to fold all elements in the queue.
    /// The `op` satisfy semigroup.
    pub struct FoldableQueue<T, F> {
        front: Vec<(T, T)>,
        back: Vec<(T, T)>,
        op: F,
    }
    impl<T, F> FoldableQueue<T, F>
    where
        T: Copy + Debug,
        F: Fn(T, T) -> T,
    {
        /// Make a new struct `FoldableQueue`
        pub fn new(op: F) -> FoldableQueue<T, F> {
            FoldableQueue {
                front: Vec::new(),
                back: Vec::new(),
                op,
            }
        }
        /// Return a folded value op(a1, a2, a3, a4, a5...,a_n).
        pub fn fold_all(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }
            let folded_value = match (self.front.last(), self.back.last()) {
                (Some(v1), None) => Some(v1.1),
                (None, Some(v2)) => Some(v2.1),
                (Some(v1), Some(v2)) => Some((self.op)(v1.1, v2.1)),
                (None, None) => None,
            };
            folded_value
        }
        /// Clears the queue.
        pub fn clear(&mut self) {
            self.front.clear();
            self.back.clear();
        }
        /// Returns the number of elements in the queue.
        pub fn len(&self) -> usize {
            self.front.len() + self.back.len()
        }

        /// Returns `true` if the queue contains no elements.
        pub fn is_empty(&self) -> bool {
            self.front.is_empty() && self.back.is_empty()
        }

        /// Appends an element to the queue.
        pub fn push(&mut self, value: T) {
            let folded_value = if let Some(&(_, folded_value)) = self.back.last() {
                (self.op)(folded_value, value)
            } else {
                value
            };
            self.back.push((value, folded_value));
        }
        /// If the first stack is empty, Pushes all elements in the back stack to the first stack.
        fn push_to_first(&mut self) {
            if self.front.is_empty() {
                let (v, _) = self.back.pop().expect("back stack is empty");
                self.front.push((v, v));
                while let Some((value, _)) = self.back.pop() {
                    let folded_value =
                        (self.op)(value, self.front.last().expect("front stack is empty").1);
                    self.front.push((value, folded_value));
                }
            }
        }
        /// Returns a first element in the queue.
        pub fn front(&mut self) -> Option<&T> {
            if self.is_empty() {
                return None;
            }
            self.push_to_first();
            self.front.last().map(|p| &p.0)
        }
        /// Removes a first element from the queue.
        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }
            self.push_to_first();
            self.front.pop().map(|p| p.0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::foldable_queue;
    #[test]
    fn test_foldable_queue() {
        let mut fque = foldable_queue::FoldableQueue::new(|x: usize, y: usize| x + y);

        let n = 20;
        for i in 0..n {
            fque.push(i);
        }
        let mut folded_value = fque.fold_all().expect("no value");
        assert_eq!(folded_value, n * (n - 1) / 2);
        assert_eq!(fque.len(), n);

        while let Some(v) = fque.pop() {
            folded_value -= v;
            assert_eq!(fque.fold_all().unwrap_or(0), folded_value);
        }
    }
    #[test]
    fn test_sliding_minimum_elements() {
        let mut fque = foldable_queue::FoldableQueue::new(std::cmp::min);
        let k = 3;
        let v = vec![1, 7, 7, 4, 8, 1, 6];
        let n = v.len();
        v.iter().take(k).for_each(|x| fque.push(*x));
        let mut results = vec![];
        for i in k..n {
            assert_eq!(fque.len(), k);
            results.push(fque.fold_all().expect("no value"));
            fque.push(v[i]);
            let x = fque.pop();
            assert_eq!(Some(v[i - k]), x);
        }

        results.push(fque.fold_all().expect("no value"));
        assert_eq!(results, vec![1, 4, 4, 1, 1]);
    }
}
