use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        mv: [usize; n],
    };
    let sum = mv.iter().sum::<usize>();
    let min_m = mv.iter().min().unwrap();
    let ans = n + (x - sum) / min_m;
    println!("{}", ans);
}
