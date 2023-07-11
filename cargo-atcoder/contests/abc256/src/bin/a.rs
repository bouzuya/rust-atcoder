use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = 2_usize.pow(n as u32);
    println!("{}", ans);
}
