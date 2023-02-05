use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let sum = a.iter().sum::<usize>();
    let ans = a.into_iter().filter(|a_i| a_i * 4 * m >= sum).count() >= m;
    println!("{}", if ans { "Yes" } else { "No" });
}
