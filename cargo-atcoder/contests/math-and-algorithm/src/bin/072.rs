use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut max = 0;
    for x in 1..=b {
        if (b / x) - ((a - 1) / x) >= 2 {
            max = max.max(x);
        }
    }
    let ans = max;
    println!("{}", ans);
}
