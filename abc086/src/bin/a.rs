use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a * b) % 2 == 0;
    println!("{}", if ans { "Even" } else { "Odd" });
}
