use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 8],
    };

    for i in 0..8 {
        let n = 8 - i;
        for j in 0..8 {
            let a = (b'a' + j as u8) as char;
            if s[i][j] == '*' {
                println!("{}{}", a, n);
            }
        }
    }
}
