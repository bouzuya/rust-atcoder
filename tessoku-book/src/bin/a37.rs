use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: usize,
        a: [usize; n],
        c: [usize; m]
    };
    let sum_c = c.iter().sum::<usize>();
    let ans = a.iter().map(|a_i| a_i * m).sum::<usize>() + b * n * m + sum_c * n;
    println!("{}", ans);
}
