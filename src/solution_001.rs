fn sum_of_multiples_of_three_or_five_below(n: u32) -> u32 {
    (1..n)
        .filter(|&n| n % 3 == 0 || n % 5 == 0)
        .fold(0, |sum, x| sum + x)
}

pub fn solve() -> String {
    sum_of_multiples_of_three_or_five_below(1000).to_string()
}
