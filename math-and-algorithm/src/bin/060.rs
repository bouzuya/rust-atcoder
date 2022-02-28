use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    // 1 : First
    // 2 : First
    // 3 : First
    // 4 : Second
    // 5 : First
    // 6 : First
    // 7 : First
    // 8 : Second
    let ans = (n % 4) != 0;
    println!("{}", if ans { "First" } else { "Second" });
}
