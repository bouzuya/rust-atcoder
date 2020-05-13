use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        h_h: usize,
        w_w: usize,
    };
    let ans = (h - h_h) * (w - w_w);
    println!("{}", ans);
}
