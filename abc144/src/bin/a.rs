use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if (1..=9).contains(&a) && (1..=9).contains(&b) {
        a * b
    } else {
        -1
    };
    println!("{}", ans);
}
