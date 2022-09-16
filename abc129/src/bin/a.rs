use proconio::input;

fn main() {
    input! {
        p: usize,
        q: usize,
        r: usize,
    };
    let ans = (p + q).min(p + r).min(q + r);
    println!("{}", ans);
}
