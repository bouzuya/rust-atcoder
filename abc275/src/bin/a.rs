use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };
    let max_h = h.iter().max().unwrap();
    let ans = h.iter().position(|h_i| h_i == max_h).unwrap() + 1;
    println!("{}", ans);
}
