struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn even_fibs(cap: u32) -> u32 {
    (Fibonacci { curr: 1, next: 1})
        .take_while(|&n| n < cap)
        .filter(|&n| n % 2 == 0)
        .fold(0, |sum, x| sum + x)
}

fn main() {
    println!("{}", even_fibs(4000000));
}
