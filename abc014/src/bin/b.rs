use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [i64; n],
    };
    let mut sum = 0;
    for i in 0..n {
        if (x >> i) & 1 == 1 {
            sum += a[i];
        }
    }
    let ans = sum;
    println!("{}", ans);
}
