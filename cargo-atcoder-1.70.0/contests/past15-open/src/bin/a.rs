use proconio::input;

fn main() {
    input! {
        a: usize,
        t: usize,
    };
    let ans = 5 * a + t;
    println!("{}", ans);
}
