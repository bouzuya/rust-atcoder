use proconio::input;

fn main() {
    input! {
        _m1: usize,
        _d1: usize,
        _m2: usize,
        d2: usize,
    };
    let ans = d2 == 1;
    println!("{}", if ans { "1" } else { "0" });
}
