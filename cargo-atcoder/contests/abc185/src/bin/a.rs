use proconio::input;

fn main() {
    input! {
        a: [i64; 4],
    };
    let ans = *a.iter().min().unwrap();
    println!("{}", ans);
}
