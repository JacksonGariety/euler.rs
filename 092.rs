struct SquareDigitChain {
    first: bool,
    curr: u32,
}

fn sum_square_digits(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += [0, 1, 4, 9, 16, 25, 36, 49, 64, 81][(n % 10) as usize]; // use pre-computed squares 0 through 9
        n /= 10; // remove one digit from the number
    }
    return sum
}

impl SquareDigitChain {
    fn new(start: u32) -> SquareDigitChain {
        SquareDigitChain { first: true, curr: start }
    }
}

impl Iterator for SquareDigitChain {
    type Item = u32;
    
    fn next(&mut self) -> Option<u32> {            
        let old_first = self.first;
        self.first = false;

        if (self.curr == 89 || self.curr == 1) && !old_first { None }
        else {
            self.curr = sum_square_digits(self.curr);
            Some(self.curr)
        }
    }
}

fn square_digit_chain(max:u32, end:u32) -> usize {
    (1..max) // make a range of each number below that of the problem statement
        .filter(|&i| SquareDigitChain::new(i).last() == Some(end) )
        .count() // count the number of items remaining in the range
}

fn main() {
    println!("{}", square_digit_chain(10000000, 89))
}
