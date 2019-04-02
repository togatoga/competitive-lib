pub struct UnionFind {
    parent: Vec<i32>,
    size: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> UnionFind {
        let parent = vec![-1; size];
        UnionFind {
            parent: parent,
            size: size,
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) {
        let x_root = self.root(x);
        let y_root = self.root(y);
        //different set
        if x_root != y_root {
            let size1 = self.parent[x_root];
            let size2 = self.parent[y_root];
            //merge smaller one for bigger one
            if size1 >= size2 {
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
        return self.root(x) == x;
    }
    pub fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        return self.root(x) == self.root(y);
    }
    pub fn root(&mut self, x: usize) -> usize {
        //x doesn't have a parent. x is the root
        if self.parent[x] < 0 {
            return x;
        }
        let parent = self.parent[x] as usize;
        let root = self.root(parent);
        self.parent[x] = root as i32;
        return root;
    }
    pub fn set_size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        let set_size = -1 * self.parent[root];
        return set_size as usize;
    }
    pub fn size(&self) -> usize {
        return self.size;
    }
}
