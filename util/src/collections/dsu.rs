pub struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

// Uses the implementation described in https://en.wikipedia.org/wiki/Disjoint-set_data_structure
impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }
        x
    }

    pub fn union(&mut self, x: usize, y: usize) -> usize {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x != y {
            if self.size[x] < self.size[y] {
                std::mem::swap(&mut x, &mut y);
            }

            self.parent[y] = x;
            self.size[x] += self.size[y];
        }

        self.size[x]
    }

    pub fn set_size(&mut self, x: usize) -> usize {
        let i = self.find(x);
        self.size[i]
    }
}
