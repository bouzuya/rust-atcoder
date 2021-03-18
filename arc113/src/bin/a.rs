use proconio::input;

fn main() {
    input! {
        k: usize,
    };
    let mut count = 0;
    for a in 1..=k {
        for b in 1..=k / a {
            count += k / (a * b);
        }
    }
    let ans = count;
    println!("{}", ans);
}
