use proconio::input;

fn main() {
    input! {
        (a, b, _c): (i64, i64, i64),
        k: usize,
    };
    let ans = if k % 2 == 0 { a - b } else { -a + b };
    println!("{}", ans);
}
