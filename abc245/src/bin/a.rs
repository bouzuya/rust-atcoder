use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let ans = a < c || (a == c && b <= d);
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
