use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = a.into_iter().any(|a_i| a_i % 2 != 0);
    println!("{}", if ans { "first" } else { "second" });
}
