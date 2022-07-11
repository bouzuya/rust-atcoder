use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let a = if a == 1 { 14 } else { a };
    let b = if b == 1 { 14 } else { b };
    let ans = match a.cmp(&b) {
        std::cmp::Ordering::Less => "Bob",
        std::cmp::Ordering::Equal => "Draw",
        std::cmp::Ordering::Greater => "Alice",
    };
    println!("{}", ans);
}
