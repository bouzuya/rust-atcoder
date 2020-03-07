use proconio::input;

fn main() {
    input! {
        s: String
    };
    let ac = s.chars().filter(|&c| c == 'A').fold(0, |acc, _| acc + 1);
    let ans = if 1 <= ac && ac < 3 { "Yes" } else { "No" };
    println!("{}", ans);
}
