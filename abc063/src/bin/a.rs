use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if a + b >= 10 {
        "error".into()
    } else {
        format!("{}", a + b)
    };
    println!("{}", ans);
}
