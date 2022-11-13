use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let max = a.iter().max().unwrap();
    let ans = a.iter().position(|a_i| a_i == max).unwrap() + 1;
    println!("{}", ans);
}
