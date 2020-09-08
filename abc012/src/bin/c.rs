use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut all = 0;
    for a in 1..=9 {
        for b in 1..=9 {
            all += a * b;
        }
    }
    let x = all - n;
    for a in 1..=9 {
        for b in 1..=9 {
            if a * b == x {
                println!("{} x {}", a, b);
            }
        }
    }
}
