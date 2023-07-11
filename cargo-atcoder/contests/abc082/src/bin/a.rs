use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let avg = (a + b) / 2;
    let ans = if avg * 2 == a + b { avg } else { avg + 1 };
    println!("{}", ans);
}
