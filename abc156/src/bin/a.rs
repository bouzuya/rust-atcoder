use proconio::input;

fn main() {
    input! {
        n: usize,
        r: usize
    };
    let ans = if n >= 10 { r } else { r + (100 * (10 - n)) };
    println!("{}", ans);
}
