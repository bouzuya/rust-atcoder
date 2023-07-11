use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let count = a.iter().copied().filter(|a_i| a_i != &0).count();
    let sum = a.iter().sum::<usize>();
    let ans = (sum + count - 1) / count;
    println!("{}", ans);
}
