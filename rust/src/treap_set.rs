pub mod treap_set {

    #[derive(Debug, Clone)]
    struct Node<T> {
        key: T,
        left: Option<Box<Node<T>>>,
        right: Option<Box<Node<T>>>,
        priority: u32,
        size: usize,
    }
    impl<T: PartialOrd> Node<T> {
        fn new(key: T, priority: u32) -> Node<T> {
            Node {
                key,
                left: None,
                right: None,
                priority,
                size: 1,
            }
        }
        fn size(&self) -> usize {
            self.size
        }
        fn update_size(&mut self) {
            self.size = size(&self.left) + size(&self.right) + 1;
        }

        fn find(&self, value: &T) -> (bool, usize) {
            let left_size = size(&self.left);
            match self.key.cmp(value) {
                _ => {}    
            }
            todo!()
        }
    }




    fn size<T>(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |node| node.size())
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

    impl<T> TreapSet<T> {
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
