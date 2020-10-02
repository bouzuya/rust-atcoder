use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        x: [usize; n]
    };
    let mut sum = 0_usize;
    let mut x_c = x[0];
    for &x_i in x.iter().skip(1) {
        let d = x_i - x_c;
        sum += std::cmp::min(d * a, b);
        x_c = x_i;
    }
    let ans = sum;
    println!("{}", ans);
}
