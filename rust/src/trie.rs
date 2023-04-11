use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod trie {
    const ALPHABET: usize = 26;
    /// `TrieAlpha` accepts only lowercase alphabets('a'..='z').
    #[derive(Debug, Clone, Default)]
    pub struct TrieAlpha {
        count: usize,
        childs: Vec<Option<Box<TrieAlpha>>>,
    }
    impl TrieAlpha {
        /// Makes a new `TrieAlpha`.
        pub fn new() -> Self {
            TrieAlpha {
                count: 0,
                childs: vec![None; ALPHABET],
            }
        }
        /// Addes a string `s`(a slice of char) to the `TrieAlpha`.
        /// Creates a new `TrieAlpha` if there isn't a child `TrieAlpha`
        pub fn add(&mut self, s: &[char]) {
            if s.is_empty() {
                self.count += 1;
                return;
            }
            let c = s[0];
            assert!(c.is_ascii_lowercase() && c.is_ascii_alphabetic());
            let idx = c as usize - 'a' as usize;
            if self.childs[idx].is_none() {
                self.childs[idx] = Some(Box::new(TrieAlpha::new()));
            }
            self.childs[idx].as_mut().expect("no child").add(&s[1..]);
        }

        fn count_prefixed_with_inner(&self) -> usize {
            let mut child_count = 0;
            for i in 0..ALPHABET {
                if let Some(child) = self.childs[i].as_ref() {
                    child_count += child.as_ref().count_prefixed_with_inner();
                }
            }
            child_count + self.count
        }

        /// Returns the number of string prefixed with `s`.
        /// `trie.add(&['a', 'b', 'c'])`.
        /// `trie.add(&['a', 'b'])`.
        /// `trie.add(&['a'])`.
        /// `trie.count_with_prefix(&['a'])` => 3
        pub fn count_prefixed_with(&self, s: &[char]) -> usize {
            if let Some(root) = self.seek(s) {
                root.count_prefixed_with_inner()
            } else {
                0
            }
        }
        /// Returns the number of string that is a prefix of `s`.
        /// Example
        /// `trie.add(&['a', 'b', 'c'])`.
        /// `trie.add(&['a', 'b'])`.
        /// `trie.add(&['a'])`.
        /// `trie.count_prefix(&['a', 'b', 'c'])` => 3
        pub fn count_prefix_of(&self, s: &[char]) -> usize {
            if s.is_empty() {
                return self.count;
            }
            let c = s[0];
            assert!(c.is_ascii_lowercase() && c.is_ascii_alphabetic());
            let idx = c as usize - 'a' as usize;
            let child_count = if let Some(child) = self.childs[idx].as_ref() {
                child.as_ref().count_prefix_of(&s[1..])
            } else {
                0
            };
            child_count + self.count
        }
        /// Returns a ref `TrieAlpha` if exists by following `s` chars one bye one.
        pub fn seek(&self, s: &[char]) -> Option<&TrieAlpha> {
            if s.is_empty() {
                return Some(self);
            }
            let c = s[0];
            assert!(c.is_ascii_lowercase() && c.is_ascii_alphabetic());
            let idx = c as usize - 'a' as usize;
            self.childs[idx].as_ref()?;
            self.childs[idx].as_ref().expect("no child").seek(&s[1..])
        }

        /// Returns the number of string `s` in `TrieAlpha`.
        pub fn count(&self, s: &[char]) -> usize {
            if s.is_empty() {
                return self.count;
            }
            let c = s[0];
            assert!(c.is_ascii_lowercase() && c.is_ascii_alphabetic());
            let idx = c as usize - 'a' as usize;
            if let Some(child) = self.childs[idx].as_ref() {
                child.as_ref().count(&s[1..])
            } else {
                0
            }
        }
        
    }
}
#[cfg(test)]
mod test {
    use super::trie::TrieAlpha;

    #[test]
    fn test_trie_alpha() {
        let mut trie = TrieAlpha::new();

    
        assert_eq!(trie.count(&['a', 'b', 'c']), 0);
        trie.add(&['a', 'b', 'c']);
        assert_eq!(trie.count(&['a', 'b', 'c']), 1);

    
        trie.add(&['a', 'b']);
        // abc, ab        
        trie.add(&['a']);
        // abc, ab, a       
        trie.add(&['a', 'c']);
        // abc, ab, a, ac
        trie.add(&['b', 'c']);
        // abc, ab, a, ac, bc


        // abc, ab, a
        assert_eq!(trie.count_prefix_of(&['a', 'b', 'c']), 3);        
        // ac, a
        assert_eq!(trie.count_prefix_of(&['a', 'c']), 2);


        trie.add(&['a', 'b']);
        assert_eq!(trie.count(&['a', 'b']), 2);        
    }
}
