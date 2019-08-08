//! Problem 14: Longest Collatz sequence

use std::collections::HashMap;

pub fn solution() -> usize {
    let mut cache = CollatzLengthCache {
        cache: [(1, 1)].iter().cloned().collect::<HashMap<_, _>>(),
    };

    let mut max = 0;
    for n in 500_000..1_000_000 {
        let len = cache.len(n);
        if len > max {
            max = len;
        }
    }

    max
}

struct CollatzLengthCache {
    cache: HashMap<usize, usize>,
}

impl CollatzLengthCache {
    fn len(&mut self, n: usize) -> usize {
        let mut len = 0;
        let mut current = n;
        let mut visited = Vec::new();

        while !self.cache.contains_key(&current) {
            current = match current % 2 {
                0 => n / 2,
                _ => 3*n + 1,
            };

            visited.push(current);
            len += 1;
        }

        let found_len = self.cache[&current];
        let mut l = 0;
        for v in visited.iter().rev() {
            self.cache.insert(*v, found_len + l);
            l += 1;
        }

        found_len + len
    }
}

#[test]
fn test() {
    assert_eq!(solution(), 837_799);
}
