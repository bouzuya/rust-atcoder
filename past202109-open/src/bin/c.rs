use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let ans = a.into_iter().filter(|a_i| a_i == &x).count();
    println!("{}", ans);
}
