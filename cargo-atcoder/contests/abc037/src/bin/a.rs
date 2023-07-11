use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = c / a.min(b);
    println!("{}", ans);
}
