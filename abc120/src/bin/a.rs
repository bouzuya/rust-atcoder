use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = (b / a).min(c);
    println!("{}", ans);
}
