use proconio::input;

fn main() {
    input! {
        r: i64,
        g: i64,
    };
    let ans = r + (g - r) * 2;
    println!("{ans}");
}
