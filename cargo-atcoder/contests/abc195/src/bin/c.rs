use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = if n < 1_000 {
        0
    } else if n < 1_000_000 {
        let a = n - (1_000 - 1);
        a * 1
    } else if n < 1_000_000_000 {
        let a = n - (1_000_000 - 1);
        let b = n - a - (1_000 - 1);
        a * 2 + b * 1
    } else if n < 1_000_000_000_000 {
        let a = n - (1_000_000_000 - 1);
        let b = n - a - (1_000_000 - 1);
        let c = n - a - b - (1_000 - 1);
        a * 3 + b * 2 + c * 1
    } else if n < 1_000_000_000_000_000 {
        let a = n - (1_000_000_000_000 - 1);
        let b = n - a - (1_000_000_000 - 1);
        let c = n - a - b - (1_000_000 - 1);
        let d = n - a - b - c - (1_000 - 1);
        a * 4 + b * 3 + c * 2 + d * 1
    } else if n == 1_000_000_000_000_000 {
        let a = n - 1 - (1_000_000_000_000 - 1);
        let b = n - 1 - a - (1_000_000_000 - 1);
        let c = n - 1 - a - b - (1_000_000 - 1);
        let d = n - 1 - a - b - c - (1_000 - 1);
        a * 4 + b * 3 + c * 2 + d * 1 + 5
    } else {
        unreachable!("invalid")
    };
    println!("{}", ans);
}
