use proconio::input;

fn main() {
    input! {
        n: usize,
        abcde: [usize; 5],
    };
    let x = abcde.iter().copied().min().unwrap();
    let count = (n + x - 1) / x;
    let ans = count + 4;
    println!("{}", ans);
}
