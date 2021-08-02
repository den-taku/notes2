// sieve :: [Int] -> [Int]
// sieve (p:xs) = p : sieve [x | x <- xs, x `mod` p /= 0]
fn main() {
    let primes = Prime::new();
    println!("{}", primes.take_while(|&x| x < 200_000).last().unwrap());
}

struct Prime {
    iterator: Box<dyn Iterator<Item = i32>>,
    primes: Vec<i32>,
}

impl Prime {
    fn new() -> Self {
        Self {
            iterator: Box::new(2..),
            primes: Vec::new(),
        }
    }
}

impl Iterator for Prime {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(e) = self.iterator.next() {
            if self.primes.iter().all(|p| e % p != 0) {
                self.primes.push(e);
                return Some(e);
            }
        }
        None
    }
}
