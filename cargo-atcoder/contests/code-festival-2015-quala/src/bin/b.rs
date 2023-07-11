use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut sum = 0;
    for a_i in a {
        sum *= 2;
        sum += a_i;
    }
    let ans = sum;
    println!("{}", ans);
}
