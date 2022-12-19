use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    a.reverse();
    let ans = a[0] + a[1];
    println!("{}", ans);
}
