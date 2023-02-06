use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
    };
    let ans = (r * 100 + g * 10 + b) % 4 == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
