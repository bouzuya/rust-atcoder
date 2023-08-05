use proconio::input;

fn main() {
    input! {
        l: usize,
        r: usize,
        d: usize,
    };
    let ans = (r / d) - ((l - 1) / d);
    println!("{}", ans);
}
