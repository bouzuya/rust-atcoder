use proconio::input;

fn main() {
    input! {
        r: u64,
    };
    let ans = if r < 1200 {
        "ABC"
    } else if r < 2800 {
        "ARC"
    } else {
        "AGC"
    };
    println!("{}", ans);
}
