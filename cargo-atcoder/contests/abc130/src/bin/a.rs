use proconio::input;

fn main() {
    input! {
        x: usize,
        a: usize,
    };
    let ans = if x < a { 0 } else { 10 };
    println!("{}", ans);
}
