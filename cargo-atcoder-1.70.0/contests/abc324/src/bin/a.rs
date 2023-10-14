use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let a_0 = a[0];
    let ans = a.iter().copied().all(|a_i| a_0 == a_i);
    println!("{}", if ans { "Yes" } else { "No" });
}
