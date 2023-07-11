use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    if n == 0 {
        println!("{}", 0);
        return;
    }
    let mut count = if n % 2 == 0 {
        0
    } else {
        if vec!['i', 'w'].contains(&s[n / 2]) {
            0
        } else {
            1
        }
    };
    for i in 0..n / 2 {
        let j = n - i - 1;
        match (s[i], s[j]) {
            ('i', 'i') => continue,
            ('w', 'w') => continue,
            ('(', ')') => continue,
            (')', '(') => continue,
            _ => count += 1,
        }
    }
    let ans = count;
    println!("{}", ans);
}
