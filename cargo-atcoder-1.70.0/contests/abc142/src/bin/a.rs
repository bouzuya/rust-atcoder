use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let (mut e, mut o) = (0, 0);
    for i in 1..=n {
        if i % 2 == 0 {
            e += 1;
        } else {
            o += 1;
        }
    }
    let ans = o as f64 / n as f64;
    println!("{}", ans);
}
