use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let d2 = x % 100;
    let c = d2 / 5 + if d2 % 5 != 0 { 1 } else { 0 };
    let ans = x >= (c * 100 + d2);
    println!("{}", if ans { "1" } else { "0" });
}
