use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars,
    };
    let mut count = 0;
    for i in 0..=999 {
        let d = format!("{:03}", i).chars().collect::<Vec<char>>();
        let mut j = 0;
        for &s_i in s.iter() {
            if j == 3 {
                break;
            }
            if d[j] == s_i {
                j += 1;
            }
        }
        if j == 3 {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
