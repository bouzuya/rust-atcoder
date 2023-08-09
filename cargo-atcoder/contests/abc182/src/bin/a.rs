use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = 2 * a + 100 - b;
    println!("{}", ans);
}
