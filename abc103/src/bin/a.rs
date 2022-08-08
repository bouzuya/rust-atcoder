use proconio::input;

fn main() {
    input! {
        mut a: [i64; 3],
    };
    a.sort();
    let ans = a[2] - a[1] + a[1] - a[0];
    println!("{}", ans);
}
