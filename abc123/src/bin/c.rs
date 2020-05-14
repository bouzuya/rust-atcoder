use proconio::input;

fn main() {
    input! {
        n: i64,
        abcde: [i64; 5],
    };
    let m = abcde.iter().min().unwrap();
    let ans = (n + (m - 1)) / m + 4;
    println!("{}", ans);
}
