use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let sum = p.iter().copied().sum::<usize>();
    let max = p.iter().copied().max().unwrap();
    let ans = sum - max + max / 2;
    println!("{}", ans);
}
