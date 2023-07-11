use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = match (a + b).cmp(&(c + d)) {
        std::cmp::Ordering::Less => "Right",
        std::cmp::Ordering::Equal => "Balanced",
        std::cmp::Ordering::Greater => "Left",
    };
    println!("{}", ans);
}
