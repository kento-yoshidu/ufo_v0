use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent: Vec::new(),
            size: Vec::new(),
        }
    }

    pub fn add(&mut self) -> usize {
        let index = self.parent.len();
        self.parent.push(index);
        self.size.push(1);
        index
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);

            self.parent[x] = root;
        }

        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }

        self.parent[y] = x;
        self.size[x] += self.size[y];

        true
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);

        self.size[root]
    }

    pub fn reset(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
            self.size[i] = 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_returns_sequential_indices() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        assert_eq!(a, 0);
        assert_eq!(b, 1);
        assert_eq!(c, 2);
    }

    #[test]
    fn unite_makes_same_true() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        assert!(!uf.same(a, b));
        assert!(!uf.same(a, c));

        uf.unite(a, b);

        assert!(uf.same(a, b));
        assert!(!uf.same(a, c));
    }

    #[test]
    fn size_reflects_set_size_after_unite() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        assert_eq!(uf.size(a), 1);

        uf.unite(a, b);

        assert_eq!(uf.size(a), 2);
        assert_eq!(uf.size(b), 2);
        assert_eq!(uf.size(c), 1);
    }

    #[test]
    fn unite_is_transitive() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        uf.unite(a, b);
        uf.unite(b, c);

        assert!(uf.same(a, c));
    }

    #[test]
    fn unite_twice_is_idempotent() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();

        assert!(uf.unite(a, b));
        assert!(!uf.unite(a, b));
        assert!(!uf.unite(b, a));
        assert_eq!(uf.size(a), 2);
    }

    #[test]
    fn same_is_true_for_identical_element() {
        let mut uf = UnionFind::new();

        let a = uf.add();

        assert!(uf.same(a, a));
    }

    #[test]
    fn unite_attaches_smaller_tree_under_larger_root() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        uf.unite(a, b); // {a, b}（size 2、代表元 a）
        uf.unite(c, a); // c（size 1）が {a, b} 側にぶら下がる

        assert_eq!(uf.find(a), a);
        assert_eq!(uf.find(b), a);
        assert_eq!(uf.find(c), a);
    }

    #[test]
    fn find_compresses_path_without_changing_result() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();
        let d = uf.add();

        uf.unite(a, b);
        uf.unite(c, d);
        uf.unite(b, d); // d -> c -> a という深さ2の木ができる

        assert_eq!(uf.parent[d], c);

        let root = uf.find(d);

        assert_eq!(root, a);
        assert_eq!(uf.parent[d], a); // 経路圧縮で直接 root を指す
        assert_eq!(uf.find(d), a); // 圧縮後も結果は変わらない
    }

    #[test]
    fn reset_makes_every_element_its_own_singleton() {
        let mut uf = UnionFind::new();

        let a = uf.add();
        let b = uf.add();
        let c = uf.add();

        uf.unite(a, b);
        uf.unite(b, c);

        uf.reset();

        assert!(!uf.same(a, b));
        assert!(!uf.same(b, c));
        assert_eq!(uf.size(a), 1);
        assert_eq!(uf.size(b), 1);
        assert_eq!(uf.size(c), 1);
    }
}
