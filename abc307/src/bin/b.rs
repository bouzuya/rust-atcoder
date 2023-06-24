use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut t = s[i].clone();
            t.extend(s[j].iter().copied());
            let mut ok = true;
            for k in 0..t.len() / 2 {
                if t[k] != t[t.len() - k - 1] {
                    ok = false;
                    break;
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
