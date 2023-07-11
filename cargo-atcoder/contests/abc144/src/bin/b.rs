use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for a in 1..=9 {
        for b in 1..=9 {
            if a * b == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
