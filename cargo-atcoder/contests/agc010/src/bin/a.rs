use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let count_odd = a.into_iter().filter(|a_i| a_i % 2 != 0).count();
    let ans = count_odd % 2 == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
