use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = x / 11 * 2
        + match x % 11 {
            0 => 0,
            1..=6 => 1,
            7..=10 => 2,
            _ => unreachable!(),
        };
    println!("{}", ans);
}
