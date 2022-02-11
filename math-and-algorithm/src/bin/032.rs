use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let ans = a.contains(&x);
    println!("{}", if ans { "Yes" } else { "No" });
}
