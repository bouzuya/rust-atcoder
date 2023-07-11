use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = (a + b) % 24;
    println!("{}", ans);
}
