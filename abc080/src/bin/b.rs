use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ans = n % n
        .to_string()
        .chars()
        .map(|c| (c as u8 - '0' as u8) as usize)
        .sum::<usize>()
        == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
