use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        k: usize,
    };
    let max = a.max(b).max(c);
    let ans = a + b + c - max + max * 2_usize.pow(k as u32);
    println!("{}", ans);
}
