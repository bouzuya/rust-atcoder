use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: [usize; n],
    };
    let ans = c.iter().position(|c_i| c_i == &(a + b)).unwrap() + 1;
    println!("{}", ans);
}
