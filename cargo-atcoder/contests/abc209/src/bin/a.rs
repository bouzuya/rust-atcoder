use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if b > a { b - a + 1 } else { 0 };
    println!("{}", ans);
}
