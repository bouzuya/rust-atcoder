use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    };
    h.sort();
    let mut min = 1_usize << 60;
    for i in 0..n - k + 1 {
        min = min.min(h[i + k - 1] - h[i]);
    }
    let ans = min;
    println!("{}", ans);
}
