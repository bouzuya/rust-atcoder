use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    let ans = a.iter().max().unwrap() - a.iter().min().unwrap();
    println!("{}", ans);
}
