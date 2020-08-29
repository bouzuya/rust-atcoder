use proconio::input;

fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize,
    };
    let ans = s * t >= d;
    println!("{}", if ans { "Yes" } else { "No" });
}
