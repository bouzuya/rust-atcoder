use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        t: usize,
        s: [Chars; t],
    };
    for s_i in s {
        if s_i.iter().all(|&c| c == 'a') {
            println!("-1");
            continue;
        }
        if s_i.iter().collect::<String>() > "atcoder".to_string() {
            println!("0");
            continue;
        }
        for j in 0..s_i.len() {
            if s_i[j] == 'a' {
                continue;
            }
            println!(
                "{}",
                if s_i[j] <= 't' {
                    j
                } else {
                    if j == 0 {
                        0
                    } else {
                        j - 1
                    }
                }
            );
            break;
        }
    }
}
