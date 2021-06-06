use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut sum = 0_i64;
    for &a_i in a.iter() {
        if a_i <= 10 {
            continue;
        }
        sum += a_i - 10;
    }
    let ans = sum;
    println!("{}", ans);
}
