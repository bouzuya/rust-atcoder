use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut s = 0_usize;
    for a in 1..=n {
        s += (n - 1) / a;
    }
    let ans = s;
    println!("{}", ans);
}
