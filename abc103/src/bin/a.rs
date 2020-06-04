use proconio::input;

fn main() {
    input! {
        mut a: [i64; 3],
    };
    a.sort();
    let ans = (a[1] - a[0]).abs() + (a[2] - a[1]).abs();
    println!("{}", ans);
}
