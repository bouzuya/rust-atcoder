use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        h: usize,
    };
    let ans = (a + b) * h / 2;
    println!("{}", ans);
}
