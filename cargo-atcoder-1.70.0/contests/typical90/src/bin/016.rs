use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
    }

    let mut min = 9999;
    for x in 0..=9999 {
        for y in 0..=9999 {
            if a * x + b * y > n {
                break;
            }
            if (n - a * x - b * y) % c != 0 {
                continue;
            }
            let z = (n - a * x - b * y) / c;
            if x + y + z > 9999 {
                continue;
            }
            min = min.min(x + y + z);
        }
    }
    let ans = min;
    println!("{}", ans);
}
