use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = (1..=n)
        .filter(|&i| {
            !format!("{}", i).chars().any(|c| c == '7')
                && !format!("{:o}", i).chars().any(|c| c == '7')
        })
        .count();
    println!("{}", ans);
}
