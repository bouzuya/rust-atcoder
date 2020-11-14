use proconio::input;

fn main() {
    input! {
        a: usize,
    };
    let mut max = 0;
    for x in 1..a {
        let y = a - x;
        max = std::cmp::max(max, x * y);
    }
    let ans = max;
    println!("{}", ans);
}
