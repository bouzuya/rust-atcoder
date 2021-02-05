use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    };
    let n = n + if n == 100 { 1 } else { 0 };
    let ans = n * 100_usize.pow(d as u32);
    println!("{}", ans);
}
