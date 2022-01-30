use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let count_h = s.iter().copied().take_while(|&c| c == 'a').count();
    let count_t = s.iter().copied().rev().take_while(|&c| c == 'a').count();
    if count_h > count_t {
        println!("No");
    } else {
        let t = s
            .iter()
            .copied()
            .skip_while(|&c| c == 'a')
            .collect::<Vec<char>>();
        let t = t
            .iter()
            .copied()
            .rev()
            .skip_while(|&c| c == 'a')
            .collect::<Vec<char>>();
        let n = t.len();
        for i in 0..n / 2 {
            if t[i] != t[n - 1 - i] {
                println!("No");
                return;
            }
        }
        println!("Yes");
    }
}
