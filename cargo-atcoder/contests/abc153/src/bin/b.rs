use proconio::input;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n],
    };
    let ans = h <= a.into_iter().sum::<usize>();
    println!("{}", if ans { "Yes" } else { "No" });
}
