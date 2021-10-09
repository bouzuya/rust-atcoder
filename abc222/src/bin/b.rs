use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        a: [usize; n],
    };
    let ans = a.into_iter().filter(|&a_i| a_i < p).count();
    println!("{}", ans);
}
