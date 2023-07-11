use proconio::input;

fn main() {
    input! {
        h1: usize,
        m1: usize,
        h2: usize,
        m2: usize,
        k: usize,
    };
    let t1 = h1 * 60 + m1;
    let t2 = h2 * 60 + m2;
    let ans = (t2 - t1).saturating_sub(k);
    println!("{}", ans);
}
