use proconio::input;

fn main() {
    input! {
        s: String,
    };
    let ans = if s.ends_with("er") {
        "er"
    } else if s.ends_with("ist") {
        "ist"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
