use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut s: Chars
    };
    s.reverse();
    let dream = "dream".chars().rev().collect::<Vec<char>>();
    let dreamer = "dreamer".chars().rev().collect::<Vec<char>>();
    let erase = "erase".chars().rev().collect::<Vec<char>>();
    let eraser = "eraser".chars().rev().collect::<Vec<char>>();
    let n = s.len();
    let mut i = 0;
    while i < n {
        let s_i = s[i];
        match s_i {
            'm' => {
                let n_s = dream.len();
                if i + n_s <= n && &s[i..i + n_s] == dream.as_slice() {
                    i += n_s;
                } else {
                    println!("NO");
                    return;
                }
            }
            'e' => {
                let n_s = erase.len();
                if i + n_s <= n && &s[i..i + n_s] == erase.as_slice() {
                    i += n_s;
                } else {
                    println!("NO");
                    return;
                }
            }
            'r' => {
                let n_s1 = dreamer.len();
                let n_s2 = eraser.len();
                if i + n_s1 <= n && &s[i..i + n_s1] == dreamer.as_slice() {
                    i += n_s1;
                } else if i + n_s2 <= n && &s[i..i + n_s2] == eraser.as_slice() {
                    i += n_s2;
                } else {
                    println!("NO");
                    return;
                }
            }
            _ => {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
