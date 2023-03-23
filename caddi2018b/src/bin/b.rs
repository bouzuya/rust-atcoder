use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n]
    };
    let ans = ab.into_iter().filter(|&(a, b)| a >= h && b >= w).count();
    println!("{}", ans);
}
