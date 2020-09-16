use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let mut sum = 0;
    sum += match x {
        1 => 300_000,
        2 => 200_000,
        3 => 100_000,
        _ => 0,
    };
    sum += match y {
        1 => 300_000,
        2 => 200_000,
        3 => 100_000,
        _ => 0,
    };
    sum += if x == 1 && y == 1 { 400_000 } else { 0 };
    let ans = sum;
    println!("{}", ans);
}
