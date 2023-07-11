use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let sum = a.iter().sum::<usize>();
    let ans = sum <= k && (sum % 2 == k % 2);
    println!("{}", if ans { "Yes" } else { "No" });
}
