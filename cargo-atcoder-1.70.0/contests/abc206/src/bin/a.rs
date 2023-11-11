use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = match (n * 108 / 100).cmp(&206) {
        std::cmp::Ordering::Less => "Yay!",
        std::cmp::Ordering::Equal => "so-so",
        std::cmp::Ordering::Greater => ":(",
    };
    println!("{}", ans);
}
