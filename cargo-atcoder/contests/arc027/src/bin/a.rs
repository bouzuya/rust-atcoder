use proconio::input;

fn main() {
    input! {
        h: usize,
        m: usize,
    };
    let ans = (18 - 1 - h) * 60 + (60 - m);
    println!("{}", ans);
}
