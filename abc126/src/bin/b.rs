use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    // 00 <= YY <= 99
    // 01 <= MM <= 12
    let s1 = s[0..2].iter().collect::<String>().parse::<usize>().unwrap();
    let s2 = s[2..4].iter().collect::<String>().parse::<usize>().unwrap();
    let is_year1 = s1 == 0 || 12 < s1;
    let is_year2 = s2 == 0 || 12 < s2;
    let ans = match (is_year1, is_year2) {
        (false, false) => "AMBIGUOUS",
        (false, true) => "MMYY",
        (true, false) => "YYMM",
        (true, true) => "NA",
    };
    println!("{}", ans);
}
