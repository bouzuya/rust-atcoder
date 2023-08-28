use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        a: [usize; n],
    };
    let ans = p.min(q + a.into_iter().min().unwrap());
    println!("{}", ans);
}
