use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    for a in 0..=100 {
        for b in 0..=100 {
            if a * 4 + b * 7 == n {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
