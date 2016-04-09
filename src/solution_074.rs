struct FactorialDigitChain {
    n: u32, // this is the current link in the chain
    v: Vec<u32>, // the vector stores the lifetime of the chain
}

impl FactorialDigitChain {
    fn new(start: u32, terms: usize) -> FactorialDigitChain {
        (FactorialDigitChain { n: start, v: Vec::with_capacity(terms) })
    }
}

impl Iterator for FactorialDigitChain {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {
        fn sum_digit_factorials(mut n: u32) -> u32 {
            let mut sum = 0;
            while n > 0 {
                sum += [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880][(n % 10) as usize]; // use pre-computed factorials 0 through 9
                n /= 10; // remove one digit from the number
            }
            return sum
        }

        self.v.push(self.n); // update lifetime
        self.n = sum_digit_factorials(self.n); // calculate n
        if self.v.contains(&self.n) { None } // check the lifetime vector for a duplicate
        else { Some(self.n) } // return n
    }
}

pub fn solve(max:u32, terms:usize) -> usize {
    (0..max) // make a range of each number below that of the problem statement
        .filter(|&i| FactorialDigitChain::new(i, terms).count() + 1 == terms) // filter that range by the length of i's factorial digit chain
        .count() // count the number of items remaining in the range
}
