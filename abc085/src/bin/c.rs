use proconio::input;

fn main() {
    input! {
        n: u64,
        y: u64,
    };
    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - (a + b);
            if a + b + c == n && 10000 * a + 5000 * b + 1000 * c == y {
                println!("{} {} {}", a, b, c);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
