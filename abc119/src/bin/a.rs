use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let m = s[5..7].iter().collect::<String>().parse::<usize>().unwrap();
    let ans = if m <= 4 { "Heisei" } else { "TBD" };
    println!("{}", ans);
}
