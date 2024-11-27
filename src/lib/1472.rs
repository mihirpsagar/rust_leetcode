// Time taken: 23:20, 23:44, 00:02 -> Acc

struct BrowserHistory {
    arr: Vec<String>,
    curr: usize,
    end: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BrowserHistory {
    fn new(homepage: String) -> Self {
        return Self {
            arr: vec![homepage],
            curr: 0,
            end: 0,
        };
    }

    fn visit(&mut self, url: String) {
        self.curr += 1;

        if self.curr < self.arr.len() {
            self.arr[self.curr] = url;
        } else {
            self.arr.push(url);
        }
        self.end = self.curr;
    }

    fn back(&mut self, steps: i32) -> String {
        self.curr = self.curr.saturating_sub(steps as usize);
        return self.arr[self.curr].clone();
    }

    fn forward(&mut self, steps: i32) -> String {
        self.curr = std::cmp::min(self.curr + steps as usize, self.end);
        return self.arr[self.curr].clone();
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {}
}
