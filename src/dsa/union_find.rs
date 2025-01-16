use std::collections::HashMap;

struct UnionFind {
    map: HashMap<char, (char, i32)>, // (value, rank)
}

impl UnionFind {
    pub fn new() -> Self {
        return Self {
            map: HashMap::new(),
        };
    }

    // Find with path compression
    pub fn find(&mut self, val: char) -> Option<char> {
        if let Some(node) = self.map.get(&val) {
            let mut curr = node.0;
            let rank = node.1;

            if curr == val {
                return Some(curr);
            }

            while let Some(next) = self.map.get(&curr) {
                if curr == next.0 {
                    break;
                }
                curr = next.0;
            }

            if curr != val {
                self.map.insert(val, (curr, rank));
            }

            return Some(curr);
        } else {
            return None;
        }
    }

    // Union by ranking
    pub fn union(&mut self, val1: char, val2: char) {
        if self.is_same_set(val1, val2) {
            return;
        }

        if !self.map.contains_key(&val1) {
            self.map.insert(val1, (val1, 0));
        }
        if !self.map.contains_key(&val2) {
            self.map.insert(val2, (val2, 0));
        }

        let node1 = self.map.get(&val1).unwrap().clone();
        let node2 = self.map.get(&val2).unwrap().clone();

        if node1.1 < node2.1 {
            self.map.insert(node1.0, (node2.0, node1.1));
        } else if node1.1 > node2.1 {
            self.map.insert(node2.0, (node1.0, node2.1));
        } else {
            self.map.insert(node1.0, (node2.0, node1.1));
            self.map.insert(node2.0, (node2.0, node2.1 + 1));
        }
    }

    pub fn is_same_set(&mut self, val1: char, val2: char) -> bool {
        let node1 = self.find(val1);
        let node2 = self.find(val2);
        return node1.is_some() && (node1 == node2);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let mut union_find = UnionFind::new();
        union_find.union('A', 'B');
        union_find.union('B', 'D');
        union_find.union('C', 'F');
        union_find.union('C', 'I');
        union_find.union('J', 'E');
        union_find.union('G', 'J');
        union_find.union('J', 'G');
        union_find.union('G', 'J');
        union_find.union('I', 'G');
        assert_eq!(union_find.find('X'), None);
        assert_eq!(union_find.is_same_set('A', 'B'), true);
        assert_eq!(union_find.is_same_set('A', 'D'), true);
        assert_eq!(union_find.is_same_set('C', 'A'), false);
        assert_eq!(union_find.is_same_set('G', 'C'), true);
        assert_eq!(union_find.is_same_set('G', 'F'), true);
    }
}
