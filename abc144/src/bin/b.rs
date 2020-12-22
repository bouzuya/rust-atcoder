use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    for a in 1..=9 {
        for b in 1..=9 {
            if n == a * b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
