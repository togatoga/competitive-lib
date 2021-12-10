pub mod treap_set {
    use std::cmp::Ordering;

    #[derive(Debug, Clone)]
    struct Node<T> {
        key: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
        priority: u32,
        size: usize,
    }
    impl<T: Ord> Node<T> {
        fn new(key: T, priority: u32) -> Node<T> {
            Node {
                key,
                left: None,
                right: None,
                priority,
                size: 1,
            }
        }
        fn update_size(&mut self) {
            self.size = size(&self.left) + size(&self.right) + 1;
        }
        fn find(&self, value: &T) -> (bool, usize) {
            let left_size = size(&self.left);
            match self.key.cmp(value) {
                Ordering::Equal => (true, left_size),
                Ordering::Greater => self
                    .left
                    .as_ref()
                    .map_or((false, 0), |left| left.find(value)),
                Ordering::Less => self.right.as_ref().map_or((false, left_size + 1), |right| {
                    let (contained, size) = right.find(value);
                    (contained, size + left_size + 1)
                }),
            }
        }
        fn kth(&self, k: usize) -> Option<&T> {
            let left_size = size(&self.left);
            if left_size > k {
                self.left.as_ref().map_or(None, |left| left.kth(k))
            } else if left_size == k {
                Some(&self.key)
            } else {
                self.right
                    .as_ref()
                    .map_or(None, |right| right.kth(k - left_size - 1))
            }
        }
    }

    fn size<T>(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |node| node.size)
    }

    #[derive(Debug, Clone)]
    pub struct TreapSet<T> {
        root: Option<Box<Node<T>>>,
        rng: Xorshift128,
    }

    impl<T> Default for TreapSet<T> {
        fn default() -> TreapSet<T> {
            TreapSet {
                root: None,
                rng: Xorshift128::default(),
            }
        }
    }

    impl<T: Ord> TreapSet<T> {
        /// Make a new empty `TreapSet`
        pub fn new(seed: u32) -> TreapSet<T> {
            TreapSet {
                root: None,
                rng: Xorshift128::new(seed),
            }
        }

        /// Returns the number of elements in the tree
        pub fn len(&self) -> usize {
            size(&self.root)
        }

        pub fn insert(&mut self, value: T) -> bool {
            let priority = self.rng.next();
            if let Some(root) = self.root.take() {
                let (contained, k) = root.find(&value);
                if !contained {
                    self.root = Some(insert(Some(root), k, value, priority));
                    true
                } else {
                    self.root = Some(root);
                    false
                }
            } else {
                self.root = Some(Box::new(Node::new(value, priority)));
                true
            }
        }
        pub fn kth(&self, k: usize) -> Option<&T> {
            self.root.as_ref().map_or(None, |root| root.kth(k))
        }
    }

    fn insert<T: Ord>(node: Option<Box<Node<T>>>, k: usize, key: T, priority: u32) -> Box<Node<T>> {
        let (left, right) = split(node, k);
        let node = merge(left, Some(Box::new(Node::new(key, priority))));
        let mut node = merge(node, right).expect("empty node");
        node.update_size();
        node
    }

    fn merge<T: Ord>(
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
    ) -> Option<Box<Node<T>>> {
        match (left, right) {
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    left.right = merge(left.right, Some(right));
                    left.update_size();
                    Some(left)
                } else {
                    right.left = merge(Some(left), right.left);
                    right.update_size();
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }

    fn split<T: Ord>(
        node: Option<Box<Node<T>>>,
        k: usize,
    ) -> (Option<Box<Node<T>>>, Option<Box<Node<T>>>) {
        if let Some(mut node) = node {
            let left_size = size(&node.left);
            if k <= left_size {
                let (left, right) = split(node.left.take(), k);
                node.left = right;
                node.update_size();
                (left, Some(node))
            } else {
                let (left, right) = split(node.right.take(), k - left_size - 1);
                node.right = left;
                node.update_size();
                (Some(node), right)
            }
        } else {
            (None, None)
        }
    }

    /// The period is 2^128 - 1
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    pub struct Xorshift128 {
        x: u32,
        y: u32,
        z: u32,
        w: u32,
    }

    impl Default for Xorshift128 {
        fn default() -> Self {
            Xorshift128 {
                x: 123456789,
                y: 362436069,
                z: 521288629,
                w: 88675123,
            }
        }
    }
    impl Xorshift128 {
        pub fn new(seed: u32) -> Xorshift128 {
            let mut xorshift = Xorshift128::default();
            xorshift.z ^= seed;
            xorshift
        }

        pub fn next(&mut self) -> u32 {
            let t = self.x ^ (self.x << 11);
            self.x = self.y;
            self.y = self.z;
            self.z = self.w;
            self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
            self.w
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_treap_set() {}
}
