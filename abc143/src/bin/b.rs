use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [i64; n],
    };
    let mut sum = 0_i64;
    for i in 0..n {
        let x = d[i];
        for j in i + 1..n {
            let y = d[j];
            sum += x * y;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
