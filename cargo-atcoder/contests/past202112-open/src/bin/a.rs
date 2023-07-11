use proconio::input;

fn main() {
    input! {
        large_h: usize,
        large_w: usize,
        h: usize,
        w: usize,
    };
    let ans = h >= large_h && w <= large_w;
    println!("{}", if ans { "Yes" } else { "No" });
}
