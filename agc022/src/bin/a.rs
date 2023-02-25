use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut used = vec![false; 26];
    for c in s.iter().copied() {
        used[(c as u8 - b'a') as usize] = true;
    }

    for (i, b) in used.iter().copied().enumerate() {
        if !b {
            print!("{}", s.into_iter().collect::<String>());
            println!("{}", (i as u8 + b'a') as char);
            return;
        }
    }

    for (i, c) in s.iter().copied().enumerate().rev() {
        for j in (c as u8) + 1..=b'z' {
            if !used[(j - b'a') as usize] {
                for k in 0..i {
                    print!("{}", s[k]);
                }
                println!("{}", j as char);
                return;
            }
        }
        used[(c as u8 - b'a') as usize] = false;
    }
    println!("-1");
}
