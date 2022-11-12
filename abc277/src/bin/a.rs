use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        p: [usize; n],
    };
    let ans = p.iter().position(|p_i| p_i == &x).unwrap() + 1;
    println!("{}", ans);
}
