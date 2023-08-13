use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let (a, b) = (a - a.min(b), b - a.min(b));
    let ans = if c == 0 { a != 0 } else { b == 0 };
    println!("{}", if ans { "Takahashi" } else { "Aoki" });
}
