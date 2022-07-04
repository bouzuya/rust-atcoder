use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    };
    let ans = !(v * t..=v * s).contains(&d);
    println!("{}", if ans { "Yes" } else { "No" });
}
