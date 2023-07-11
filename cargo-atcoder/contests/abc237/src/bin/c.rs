use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let count_prefix_a = s.iter().take_while(|s_i| s_i == &&'a').count();
    let count_suffix_a = s.iter().rev().take_while(|s_i| s_i == &&'a').count();
    if count_prefix_a > count_suffix_a {
        println!("No");
        return;
    }
    if count_prefix_a == s.len() {
        println!("Yes");
        return;
    }

    let t = s[count_prefix_a..s.len() - count_suffix_a]
        .iter()
        .copied()
        .collect::<Vec<char>>();
    for i in 0..t.len() / 2 {
        if t[i] != t[t.len() - 1 - i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
