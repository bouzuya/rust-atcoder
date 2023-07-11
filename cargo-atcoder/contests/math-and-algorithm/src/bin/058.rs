use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
    };
    let x = x.abs() as usize;
    let y = y.abs() as usize;
    let ans = n >= x + y && (((x + y) % 2) == (n % 2));
    println!("{}", if ans { "Yes" } else { "No" });
}
