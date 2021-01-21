use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut x: Chars,
    };
    while let Some(l) = x.pop() {
        match l {
            'o' | 'k' | 'u' => continue,
            'h' => {
                if !x.is_empty() && x.pop().unwrap() == 'c' {
                    continue;
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
