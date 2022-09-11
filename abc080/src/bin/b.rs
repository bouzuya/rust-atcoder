use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let fx = n
        .to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .sum::<usize>();
    let ans = n % fx == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
