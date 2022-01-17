use cargo_snippet::snippet;
#[allow(clippy::module_inception)]
#[snippet]
pub mod union_find {
    pub struct UnionFind {
        parent: Vec<i32>,
        size: usize,
    }

    impl UnionFind {
        pub fn new(size: usize) -> UnionFind {
            let parent = vec![-1; size];
            UnionFind { parent, size }
        }
        pub fn unite(&mut self, x: usize, y: usize) {
            let x_root = self.root(x);
            let y_root = self.root(y);
            //different set
            if x_root != y_root {
                //size1 and size are negative
                //-1 -2
                let size1 = self.parent[x_root];
                let size2 = self.parent[y_root];
                //merge smaller one for bigger one
                //e.g -2 <= -1
                if size1 <= size2 {
                    self.parent[x_root] += size2;
                    //new parent
                    self.parent[y_root] = x_root as i32;
                } else {
                    self.parent[y_root] += size1;
                    //new parent
                    self.parent[x_root] = y_root as i32;
                }
                self.size -= 1;
            }
        }
        pub fn is_root(&mut self, x: usize) -> bool {
            self.root(x) == x
        }
        pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn root(&mut self, x: usize) -> usize {
            //x doesn't have a parent. x is the root
            if self.parent[x] < 0 {
                return x;
            }
            let parent = self.parent[x] as usize;
            let root = self.root(parent);
            self.parent[x] = root as i32;
            root
        }
        pub fn union_size(&mut self, x: usize) -> usize {
            let root = self.root(x);
            let set_size = -self.parent[root];
            set_size as usize
        }
        pub fn size(&self) -> usize {
            self.size
        }
    }
    pub struct UndoUnionFind {
        parent: Vec<u32>,
        size: Vec<u32>,
        stack: Vec<Option<(u32, u32)>>,
    }
    impl UndoUnionFind {
        pub fn new(n: usize) -> UndoUnionFind {
            assert!(n < std::u32::MAX as usize);
            let mut res = UndoUnionFind {
                parent: vec![0; n],
                size: vec![1; n],
                stack: vec![],
            };
            res.init();
            res
        }
        pub fn init(&mut self) {
            self.stack.clear();
            for (i, (parent, size)) in self.parent.iter_mut().zip(self.size.iter_mut()).enumerate()
            {
                *parent = i as u32;
                *size = 1;
            }
        }
        pub fn is_root(&self, x: usize) -> bool {
            x == self.root(x)
        }
        pub fn root(&self, mut x: usize) -> usize {
            assert!(x < self.parent.len());
            while self.parent[x] != x as u32 {
                x = self.parent[x] as usize;
            }
            x
        }
        pub fn is_same_set(&self, x: usize, y: usize) -> bool {
            assert!(x < self.parent.len());
            assert!(y < self.parent.len());
            self.root(x) == self.root(y)
        }
        pub fn unite(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
            assert!(x < self.parent.len());
            assert!(y < self.parent.len());
            let mut x = self.root(x);
            let mut y = self.root(y);
            if x == y {
                self.stack.push(None);
                return None;
            }
            if (self.size[x], x) < (self.size[y], y) {
                std::mem::swap(&mut x, &mut y);
            }
            self.size[x] += self.size[y];
            self.parent[y] = x as u32;
            self.stack.push(Some((x as u32, y as u32)));
            Some((x, y))
        }
        pub fn union_size(&self, x: usize) -> usize {
            assert!(x < self.parent.len());
            let r = self.root(x);
            self.size[r] as usize
        }
        pub fn undo(&mut self) -> Option<(usize, usize)> {
            self.stack.pop().unwrap().map(|(x, y)| {
                let x = x as usize;
                let y = y as usize;
                self.size[x] -= self.size[y];
                self.parent[y] = y as u32;
                (x, y)
            })
        }
        pub fn snap(&mut self) {
            self.stack.clear();
        }
        pub fn rollback(&mut self) {
            while !self.stack.is_empty() {
                self.undo();
            }
        }
    }
}
