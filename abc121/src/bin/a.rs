use proconio::input;

fn main() {
    input! {
        capital_h: usize,
        capital_w: usize,
        h: usize,
        w: usize,
    };
    let ans = (capital_h - h) * (capital_w - w);
    println!("{}", ans);
}
