use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let max = *a.iter().max().unwrap();
    let ans = a.into_iter().filter(|a_i| a_i != &max).max().unwrap();
    println!("{}", ans);
}
