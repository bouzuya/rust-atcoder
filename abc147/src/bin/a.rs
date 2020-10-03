use proconio::input;

fn main() {
    input! {
        a: [i64; 3],
    };
    let sum = a.iter().sum::<i64>();
    let ans = if sum >= 22 { "bust" } else { "win" };
    println!("{}", ans);
}
