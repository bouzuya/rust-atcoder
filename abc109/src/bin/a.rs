use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    for c in 1..=3 {
        if (a * b * c) % 2 != 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
