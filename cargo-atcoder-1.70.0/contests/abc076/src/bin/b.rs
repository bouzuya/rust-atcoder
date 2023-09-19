use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut cur = 1_usize;
    for _ in 0..n {
        cur = (cur * 2).min(cur + k);
    }
    let ans = cur;
    println!("{}", ans);
}
