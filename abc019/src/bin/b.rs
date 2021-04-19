use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut t = vec![];
    t.push(s[0]);
    let mut count = 1;
    let mut prev = s[0];
    for &s_i in s.iter().skip(1) {
        if prev == s_i {
            count += 1;
        } else {
            t.append(&mut format!("{}", count).chars().collect::<Vec<char>>());
            t.push(s_i);
            prev = s_i;
            count = 1;
        }
    }
    t.append(&mut format!("{}", count).chars().collect::<Vec<char>>());
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
