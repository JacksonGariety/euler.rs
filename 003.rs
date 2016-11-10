fn prime_factors(mut n: i64) -> Vec<i64> {
    let mut factors = vec![1];
    let mut d = 2;

    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
        if (d * d) > n {
            if n > 1 {
                factors.push(n);
            }
            break;
        }
    }
    
    return factors;
}

fn main() {
    println!("{:?}", prime_factors(600851475143).last())
}
