use proconio::input;

fn main() {
    input! {
        _c: usize,
        h: usize,
    };
    let ans = h >= 2800;
    println!("{}", if ans { "o" } else { "x" });
}
