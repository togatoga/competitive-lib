/// Sliding Window Aggregation
pub mod foldable_queue {
    /// FoldableQueue
    /// The `op` satisfy semigroup
    pub struct FoldableQueue<T, F> {
        front: Vec<(T, T)>,
        back: Vec<(T, T)>,
        op: F,
    }
    impl<T, F> FoldableQueue<T, F>
    where
        T: Copy,
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

        pub fn clear(&mut self) {
            self.front.clear();
            self.back.clear();
        }
        pub fn len(&self) -> usize {
            self.front.len() + self.back.len()
        }
        pub fn is_empty(&self) -> bool {
            self.front.is_empty() && self.back.is_empty()
        }
        pub fn push(&mut self, val: T) {
            let folded_val = if let Some(&(_, folded_val)) = self.back.last() {
                (self.op)(folded_val, val)
            } else {
                val
            };
            self.back.push((val, folded_val));
        }

        pub fn pop(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }
            if self.front.is_empty() {
                self.front
                    .push(self.back.pop().expect("back stack is empty"));
                while let Some((value, _)) = self.back.pop() {
                    let folded_value =
                        (self.op)(value, self.front.last().expect("front stack is empty").1);
                    self.front.push((value, folded_value));
                }
            }
            self.front.pop().map(|p| p.0)
        }
    }
}
