use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let yy = s[0..2].iter().collect::<String>().parse::<usize>().unwrap();
    let mm = s[2..4].iter().collect::<String>().parse::<usize>().unwrap();
    let ans = match ((1..=12).contains(&yy), (1..=12).contains(&mm)) {
        (true, true) => "AMBIGUOUS",
        (true, false) => "MMYY",
        (false, true) => "YYMM",
        (false, false) => "NA",
    };
    println!("{}", ans);
}
