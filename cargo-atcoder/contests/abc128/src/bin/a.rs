use proconio::input;

fn main() {
    input! {
        a: usize,
        p: usize,
    };
    let ans = (3 * a + p) / 2;
    println!("{}", ans);
}
