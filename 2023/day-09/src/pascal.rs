pub struct Pascal {
    cache: Vec<Vec<isize>>,
}

impl Pascal {
    pub fn new() -> Pascal {
        let first_row = vec![1];
        let second_row = vec![1, 1];
        let cache = vec![first_row, second_row];
        Pascal { cache }
    }
    pub fn choose(&mut self, n: usize, k: usize) -> isize {
        if n < self.cache.len() {
            return self.cache[n][k];
        }
        let new_row = (0..n)
            .map(|i| {
                if i == 0 || i == n - 1 {
                    1
                } else {
                    self.choose(n - 1, i - 1) + self.choose(n - 1, i)
                }
            })
            .collect();
        self.cache.push(new_row);
        self.cache[n][k]
    }
}

impl Default for Pascal {
    fn default() -> Self {
        Self::new()
    }
}

