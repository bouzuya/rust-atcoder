use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    let mut sum = 0;
    let mut a_p = a[0];
    for i in 1..n {
        let a_i = a[i];
        if a_p > a_i {
            sum += a_p - a_i;
            a[i] = a_p;
        }
        a_p = a[i];
    }
    let ans = sum;
    println!("{}", ans);
}
