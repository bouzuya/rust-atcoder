use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = ((n - m) * 100 + m * 1900) * 2_usize.pow(m as u32);
    println!("{}", ans);
}
