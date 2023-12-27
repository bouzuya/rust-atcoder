use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let max = a.iter().copied().max().unwrap();
    for a_i in a {
        let ans = (1_000_000_000_f64 * a_i as f64 / max as f64).round();
        println!("{}", ans)
    }
}
