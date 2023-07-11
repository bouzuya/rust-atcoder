use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p: [i64; n],
    };
    p.sort();
    let mut ans = 0;
    for i in 0..k {
        ans += p[i];
    }
    println!("{}", ans);
}
