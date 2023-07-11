use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut max = 0_i64;
    for i in 0..n {
        for j in i + 1..n {
            max = max.max((a[i] - a[j]).abs());
        }
    }
    let ans = max;
    println!("{}", ans);
}
