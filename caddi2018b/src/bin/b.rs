use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n],
    };
    let ans = ab
        .iter()
        .filter(|&&(a_i, b_i)| a_i >= h && b_i >= w)
        .count();
    println!("{}", ans);
}
