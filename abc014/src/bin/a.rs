use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let d = a % b;
    let ans = if d == 0 { 0 } else { b - d };
    println!("{}", ans);
}
