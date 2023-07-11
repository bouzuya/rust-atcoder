use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n >= 2 { 1 } else { 0 };
    println!("{}", ans);
}
