fn factorial(n:u32) -> u32 {
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880][n as usize]
}

fn digit_factorial_sum(mut n:u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += factorial(n % 10);
        n /= 10;
    }
    sum
}

pub fn solve(max:u32, terms:usize) -> usize {
    (1..max)
        .filter(|&i| {
            let mut v = Vec::with_capacity(terms);
            let mut start = i;
            let mut j = 0;
            while !v.contains(&start) {
                v.push(start);
                start = digit_factorial_sum(start);
                j += 1
            }
            return j == terms
        })
        .count()
}
