use proconio::input;

fn main() {
    input! {
        x: i64
    };
    let min_a = -118;
    let max_a = 119;
    for a in min_a..=max_a {
        for b in min_a - 1..=max_a - 1 {
            if a * a * a * a * a - b * b * b * b * b == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }
}
