use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut count = 0;
    let abc = "ABC".chars().collect::<Vec<char>>();
    for i in 0..n - 3 + 1 {
        if s[i..i + 3] == abc[..] {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
