use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = a.into_iter().fold(0, |acc, a_i| acc ^ a_i) != 0;
    println!("{}", if ans { "First" } else { "Second" });
}
