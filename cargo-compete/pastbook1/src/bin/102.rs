use proconio::{input, marker::Chars};

fn dfs(s: &[char], length: usize, t: &mut Vec<char>, count: &mut usize) {
    if t.len() == length {
        let mut ok = false;
        for i in 0..s.len() {
            if i + t.len() > s.len() {
                continue;
            }
            let mut m = true;
            for (j, p) in t.iter().copied().enumerate() {
                if p != '.' && p != s[i + j] {
                    m = false;
                }
            }
            ok |= m;
        }
        if ok {
            *count += 1;
        }
        return;
    }

    for i in 0_usize..26 + 1 {
        let c = if i < 26 {
            (i as u8 + b'a') as char
        } else {
            '.'
        };
        t.push(c);
        dfs(s, length, t, count);
        t.pop();
    }
}

fn main() {
    input! {
        s: Chars
    }

    let mut count = 0_usize;
    for length in 1..=3 {
        let mut c = 0_usize;
        dfs(&s, length, &mut vec![], &mut c);
        count += c;
    }
    let ans = count;
    println!("{}", ans);
}
