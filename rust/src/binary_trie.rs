use cargo_snippet::snippet;
#[snippet]
pub mod bianry_trie {
    use std::{marker::PhantomData, ops::BitAnd};

    type BNode = Box<Node>;
    fn new_node() -> BNode {
        Box::new(Node::new())
    }

    #[derive(Debug, Clone, Default)]
    struct Node {
        left: Option<BNode>,
        right: Option<BNode>,
        size: usize,
    }
    impl Node {
        pub fn new() -> Node {
            Node {
                left: None,
                right: None,
                size: 0,
            }
        }
        fn count<T: BinaryTrieValue>(&self, value: &T) -> usize {
            let mut child = self;
            for d in (0..T::bit()).rev() {
                if child.size == 0 {
                    return 0;
                }
                child = if !value.bit_set(d) {
                    if let Some(left) = child.left.as_deref() {
                        left
                    } else {
                        return 0;
                    }
                } else if let Some(right) = child.right.as_deref() {
                    right
                } else {
                    return 0;
                };
            }
            child.size
        }
        fn contains<T: BinaryTrieValue>(&self, value: &T) -> bool {
            let mut child = self;
            for d in (0..T::bit()).rev() {
                if child.size == 0 {
                    return false;
                }

                child = if !value.bit_set(d) {
                    if let Some(left) = child.left.as_deref() {
                        left
                    } else {
                        return false;
                    }
                } else if let Some(right) = child.right.as_deref() {
                    right
                } else {
                    return false;
                };
            }
            child.size > 0
        }

        fn kth_xor<T: BinaryTrieValue>(&self, mut k: usize, xor_value: T) -> Option<T> {
            let mut result = T::zero();
            let mut child = self;

            for d in (0..T::bit()).rev() {
                child = if !xor_value.bit_set(d) {
                    if let Some(node) = child.left.as_deref() {
                        if node.size > k {
                            node
                        } else {
                            // node.size <= k
                            k -= node.size;
                            result |= T::one() << d;
                            child.right.as_deref().expect("no right")
                        }
                    } else {
                        result |= T::one() << d;
                        child.right.as_deref().expect("no right")
                    }
                } else if let Some(node) = child.right.as_deref() {
                    if node.size > k {
                        node
                    } else {
                        k -= node.size;
                        result |= T::one() << d;
                        child.left.as_deref().expect("no left")
                    }
                } else {
                    result |= T::one() << d;
                    child.left.as_deref().expect("no left")
                }
            }

            Some(result)
        }
    }

    pub trait Integer {
        fn one() -> Self;
        fn zero() -> Self;
        fn bit() -> u32;
    }
    impl Integer for u32 {
        fn one() -> u32 {
            1
        }
        fn zero() -> u32 {
            0
        }
        fn bit() -> u32 {
            32
        }
    }
    impl Integer for u64 {
        fn one() -> u64 {
            1
        }
        fn zero() -> u64 {
            0
        }
        fn bit() -> u32 {
            64
        }
    }

    pub trait BinaryTrieValue:
        Copy
        + Clone
        + std::ops::Shr<u32, Output = Self>
        + std::ops::Shl<u32, Output = Self>
        + BitAnd<Self, Output = Self>
        + std::ops::BitOrAssign
        + Integer
        + std::cmp::PartialEq
    {
        fn bit_set(self, shift: u32) -> bool {
            self >> shift & Integer::one() != Integer::zero()
        }
    }

    impl BinaryTrieValue for u32 {}
    impl BinaryTrieValue for u64 {}

    /// A binary trie
    /// `BinaryTrie` is a data strcture for managing only unsinged integers(`u18`, `u16`, `u32`, `u64`) and has similar APIs that the multiset can supports.
    /// e.g. Insert a value into the set. Delete a value in the set. Get a min/max/kth value in the set.
    /// Additionally Get a min/max/kth value in the set whose all elements with xor applied.
    #[derive(Debug, Default)]
    pub struct BinaryTrie<T> {
        root: Option<BNode>,
        phantom: PhantomData<T>,
    }

    fn insert<T: BinaryTrieValue>(root: &mut BNode, value: T) -> bool {
        let mut child = root.as_mut();

        for d in (0..T::bit()).rev() {
            child.size += 1;
            child = if !value.bit_set(d) {
                if child.left.is_none() {
                    child.left = Some(new_node());
                }
                child.left.as_deref_mut().expect("no left")
            } else {
                if child.right.is_none() {
                    child.right = Some(new_node());
                }
                child.right.as_deref_mut().expect("no right")
            };
        }
        child.size += 1;
        child.size == 1
    }

    fn remove<T: BinaryTrieValue>(root: &mut BNode, value: &T) {
        let mut child = root.as_mut();

        for d in (0..T::bit()).rev() {
            child.size -= 1;
            child = if !value.bit_set(d) {
                child.left.as_deref_mut().expect("no left")
            } else {
                child.right.as_deref_mut().expect("no right")
            };
        }
        child.size -= 1;
    }

    impl<T: BinaryTrieValue> BinaryTrie<T> {
        /// Inserts a value into the set.
        /// Returns `true` if the set did not have this value present.
        pub fn insert(&mut self, value: T) -> bool {
            if self.root.is_none() {
                self.root = Some(new_node());
            }
            let root = self.root.as_mut().expect("no root");
            insert(root, value)
        }
        /// Removes a value from the set.
        /// Returns `true` if the set had this value.
        pub fn remove(&mut self, value: &T) -> bool {
            if self.root.is_none() {
                return false;
            }
            let root = self.root.as_mut().expect("no root");
            if !root.contains(value) {
                false
            } else {
                remove(root, value);
                true
            }
        }
        /// Returns `true` if the set is empty.
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }

        /// Returns the number of elements in the set.
        pub fn len(&self) -> usize {
            self.root.as_deref().map_or(0, |root| root.size)
        }

        /// Returns `true` if the set contains a `value`.
        pub fn contains(&self, value: &T) -> bool {
            self.root
                .as_deref()
                .map_or(false, |root| root.contains(value))
        }

        /// Returns the number of a `value` in the set.
        pub fn count(&self, value: &T) -> usize {
            self.root.as_deref().map_or(0, |root| root.count(value))
        }

        /// Removes all elements in the set.
        pub fn clear(&mut self) {
            self.root.take();
        }

        /// Returns a minimum value in the set.
        /// min(a1, a2, a3, ... an)
        pub fn min(&self) -> Option<T> {
            self.min_xor(T::zero())
        }

        /// Returns a maximum value in the set.
        /// max(a1, a2, a3, ... an)
        pub fn max(&self) -> Option<T> {
            self.max_xor(T::zero())
        }

        /// Returns a minimum value in the set whose all elements with xor applied.
        /// min(a1^x, a2^x, a3^x, ... an^x)
        pub fn min_xor(&self, xor_value: T) -> Option<T> {
            self.kth_xor(0, xor_value)
        }

        /// Returns a max value in the set whose all elements with xor applied.
        /// max(a1^x, a2^x, a3^x, ... an^x)
        pub fn max_xor(&self, xor_value: T) -> Option<T> {
            if self.is_empty() {
                None
            } else {
                self.kth_xor(self.len() - 1, xor_value)
            }
        }

        /// Returns a k-th(0-index) value in increasing order in the set.
        pub fn kth(&self, k: usize) -> Option<T> {
            self.kth_xor(k, T::zero())
        }

        /// Returns a k-th(0-index) value in increasing order in the set whose all elements with xor applied.
        pub fn kth_xor(&self, k: usize, xor_value: T) -> Option<T> {
            self.root
                .as_deref()
                .and_then(|root| root.kth_xor(k, xor_value))
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::{BTreeMap, BTreeSet};

    use super::bianry_trie::BinaryTrie;
    use rand::prelude::StdRng;
    use rand::prelude::*;

    #[test]
    fn test_insert_cotains() {
        let mut bt = BinaryTrie::<u32>::default();
        let mut rng = StdRng::seed_from_u64(12345);
        let mut counter = BTreeMap::default();

        let mut values = (0..1024)
            .map(|_| rng.gen_range(0, 1024))
            .collect::<Vec<u32>>();
        values.push(std::u32::MAX);

        for value in values.iter() {
            *counter.entry(value).or_insert(0) += 1;
            bt.insert(*value);
            assert!(bt.contains(value));
            assert_eq!(bt.count(value), *counter.get(&value).expect("no value"));
        }

        let value_set = values.into_iter().collect::<BTreeSet<_>>();

        for x in 0..1024 {
            let contains = value_set.contains(&x);
            assert_eq!(bt.contains(&x), contains);
        }
    }
    #[test]
    fn test_insert_remove() {
        let mut bt = BinaryTrie::<u32>::default();
        bt.insert(10);
        assert!(bt.insert(120));
        assert!(!bt.insert(120));
        assert_eq!(bt.len(), 3);
        assert_eq!(bt.count(&120), 2);
        assert!(bt.remove(&120));
        assert_eq!(bt.count(&120), 1);
        assert!(bt.remove(&120));
        assert_eq!(bt.count(&120), 0);
        assert!(!bt.contains(&120));
        assert_eq!(bt.len(), 1);

        assert!(!bt.remove(&111111));
    }
    #[test]
    fn test_kth() {
        let values: Vec<u32> = vec![0, 1, 2, 2, 5, 7, 16];
        let mut bt = BinaryTrie::<u32>::default();
        values.iter().for_each(|&x| {
            bt.insert(x);
        });

        for (i, x) in values.into_iter().enumerate() {
            assert_eq!(bt.kth(i), Some(x), "kth({})", i);
        }
        bt.clear();

        let mut rng = StdRng::seed_from_u64(114);
        let mut values = (0..10000)
            .map(|_| rng.gen_range(0, 10000))
            .collect::<Vec<u32>>();
        values.sort_unstable();
        values.iter().for_each(|x| {
            bt.insert(*x);
        });
        for (i, &x) in values.iter().enumerate() {
            assert_eq!(bt.kth(i), Some(x), "kth({})", i);
        }
        for _ in 0..500 {
            let xor_value = rng.gen_range(0, 50000);
            let mut xor_values = values.iter().map(|x| *x ^ xor_value).collect::<Vec<u32>>();
            xor_values.sort_unstable();

            for (i, &x) in xor_values.iter().enumerate() {
                assert_eq!(
                    bt.kth_xor(i, xor_value),
                    Some(x),
                    "kth_xor({}, {})",
                    i,
                    xor_value
                );
            }
            let min_value = *xor_values.iter().min().expect("no value");
            let max_value = *xor_values.iter().max().expect("no value");
            assert_eq!(bt.max_xor(xor_value).expect("no max"), max_value);
            assert_eq!(bt.min_xor(xor_value).expect("no min"), min_value);
        }
    }
}
