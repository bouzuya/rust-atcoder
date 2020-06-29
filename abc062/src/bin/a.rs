use proconio::input;

fn group(x: u8) -> u8 {
    match x {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 0,
        4 | 6 | 9 | 11 => 1,
        2 => 2,
        _ => unreachable!(),
    }
}
fn main() {
    input! {
        x: u8,
        y: u8
    };
    let ans = group(x) == group(y);
    println!("{}", if ans { "Yes" } else { "No" });
}
