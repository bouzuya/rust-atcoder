use proconio::input;

fn main() {
    input! {
        a: i64,
        op: char,
        b: i64,
    };
    let ans = match op {
        '+' => a + b,
        '-' => a - b,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
