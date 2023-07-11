use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = a.iter().sum::<usize>();
    println!("{}", ans);
}
