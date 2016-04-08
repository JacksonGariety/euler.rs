fn factorial(n:u32) -> u32 {
     return match n {
         0|1 => 1,
         2 => 2,
         3 => 6,
         4 => 24,
         5 => 120,
         6 => 720,
         7 => 5040,
         8 => 40320,
         9 => 362880,
         _ => 0
                
     }
}

fn digit_factorial_sum(mut arg:u32) -> u32 {
    let mut sum = 0;
    while arg > 0 {
        sum += factorial(arg % 10);
        arg /= 10;
    }
    sum
}

pub fn solve(max:u32, terms:usize) -> usize {
    (1..max)
        .filter(|&i| {
            let mut v: Vec<u32> = Vec::with_capacity(terms);
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
