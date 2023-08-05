use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let max = *a.iter().skip(1).max().unwrap_or(&0);
    let ans = if a[0] > max { 0 } else { max + 1 - a[0] };
    println!("{}", ans);
}
