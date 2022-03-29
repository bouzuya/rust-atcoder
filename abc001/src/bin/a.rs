use proconio::input;

fn main() {
    input! {
        h: [i64; 2]
    };
    let ans = h[0] - h[1];
    println!("{}", ans);
}
