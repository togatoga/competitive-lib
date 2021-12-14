use cargo_snippet::snippet;
#[snippet]
pub mod treap {
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
                self.left.as_ref().and_then(|left| left.kth(k))
            } else if left_size == k {
                Some(&self.key)
            } else {
                self.right
                    .as_ref()
                    .and_then(|right| right.kth(k - left_size - 1))
            }
        }
    }

    fn size<T>(node: &Option<Box<Node<T>>>) -> usize {
        node.as_ref().map_or(0, |node| node.size)
    }

    /// `TreapSet` a set based on a treap.
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

        /// Returns `true` if the tree contains no elements.
        pub fn is_empty(&self) -> bool {
            self.root.is_none()
        }

        /// Adds a value to the tree.
        ///
        /// If the tree did not have this value present, `true` is returned.
        /// Otherwise, `false` is returned.
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
        /// Removes a value from the tree. Returns whether the value was present in the tree.
        pub fn remove(&mut self, value: &T) -> bool {
            if let Some(root) = self.root.take() {
                let (contains, k) = root.find(value);
                if !contains {
                    self.root = Some(root);
                    false
                } else {
                    self.root = remove(Some(root), k);
                    true
                }
            } else {
                false
            }
        }

        /// Returns `true` if the tree contains a value.
        pub fn contains(&self, value: &T) -> bool {
            self.root.as_ref().map_or(false, |root| root.find(value).0)
        }

        /// Returns a k-th(0-index) value in decreasing order in the tree.
        pub fn kth(&self, k: usize) -> Option<&T> {
            self.root.as_ref().and_then(|root| root.kth(k))
        }

        /// Returns an index indicating what number a value is in decreasing order in the tree.
        pub fn find(&self, value: &T) -> Option<usize> {
            match self.root.as_ref() {
                Some(root) => {
                    let (contains, k) = root.find(value);
                    if contains {
                        Some(k)
                    } else {
                        None
                    }
                }
                None => None,
            }
        }
    }

    fn insert<T: Ord>(node: Option<Box<Node<T>>>, k: usize, key: T, priority: u32) -> Box<Node<T>> {
        let (left, right) = split(node, k);
        merge(merge(left, Some(Box::new(Node::new(key, priority)))), right).expect("empty node")
    }

    fn remove<T: Ord>(node: Option<Box<Node<T>>>, k: usize) -> Option<Box<Node<T>>> {
        let (left, right) = split(node, k + 1);
        let (left, _) = split(left, k);
        match merge(left, right) {
            Some(mut node) => {
                node.update_size();
                Some(node)
            }
            None => None,
        }
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

    use std::collections::BTreeSet;

    use super::treap::TreapSet;
    use rand::prelude::StdRng;
    use rand::prelude::*;

    #[test]
    fn test_treap_insert_erase() {
        let mut treap = TreapSet::new(71);
        let mut rng = StdRng::seed_from_u64(141);
        let max = 1000000;

        let mut v = (0..max).collect::<Vec<_>>();

        v.shuffle(&mut rng);
        for &i in v.iter() {
            assert!(!treap.contains(&i));
            assert!(treap.insert(i));
            assert!(!treap.insert(i));
            assert!(treap.contains(&i));
        }

        v.shuffle(&mut rng);
        for &i in v.iter() {
            assert!(treap.contains(&i));
            assert_eq!(treap.remove(&i), true);
            assert_eq!(treap.remove(&i), false);
            assert!(!treap.contains(&i));
        }
    }

    #[test]
    fn test_treap_nth() {
        let mut rng = StdRng::seed_from_u64(141);

        for _ in 0..10 {
            let mut treap = TreapSet::new(71);
            let max = 100000;
            let mut v = (0..max)
                .map(|_| rng.gen_range(0, 1_000_000_000))
                .collect::<Vec<_>>();
            v.sort_unstable();
            v.dedup();
            v.shuffle(&mut rng);
            for &i in v.iter() {
                assert!(treap.insert(i));
                assert!(!treap.insert(i));
            }
            v.sort_unstable();

            for (i, v) in v.into_iter().enumerate() {
                assert_eq!(treap.kth(i), Some(&v));
            }
        }
    }

    #[test]
    fn test_random_insertion() {
        use rand::distributions::Uniform;

        let mut rng = thread_rng();
        let mut set = BTreeSet::new();
        let mut treap = TreapSet::new(42);
        for _ in 0..2000 {
            let x = rng.gen::<i64>();

            if rng.sample(Uniform::from(0..10)) == 0 {
                // remove
                if set.contains(&x) {
                    assert!(treap.contains(&x));
                    set.remove(&x);
                    assert_eq!(treap.remove(&x), true);
                    assert_eq!(treap.remove(&x), false);
                    assert!(!treap.contains(&x));
                } else {
                    assert!(!treap.contains(&x));
                }
            } else {
                // insert
                if set.contains(&x) {
                    assert!(treap.contains(&x));
                } else {
                    assert!(!treap.contains(&x));
                    assert!(treap.insert(x));
                    assert!(!treap.insert(x));
                    set.insert(x);
                    assert!(treap.contains(&x));
                }
            }

            assert_eq!(treap.len(), set.len());

            for (i, x) in set.iter().enumerate() {
                assert_eq!(treap.kth(i), Some(x));
                assert_eq!(treap.find(x), Some(i));
            }
        }
    }
}
