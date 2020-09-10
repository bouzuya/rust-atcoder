use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        _: usize,
        d2: usize,
    };
    let ans = d2 == 1;
    println!("{}", if ans { "1" } else { "0" });
}
