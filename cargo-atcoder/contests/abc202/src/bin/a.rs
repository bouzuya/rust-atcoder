use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = (7 - a) + (7 - b) + (7 - c);
    println!("{}", ans);
}
