use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut s2 = vec![(s[0], 1)];
    for s_i in s.iter().copied().skip(1) {
        let (prev, count) = s2.pop().unwrap();
        if prev == s_i {
            s2.push((prev, count + 1));
        } else {
            s2.push((prev, count));
            s2.push((s_i, 1));
        }
    }
    let mut t2 = vec![(t[0], 1)];
    for t_i in t.iter().copied().skip(1) {
        let (prev, count) = t2.pop().unwrap();
        if prev == t_i {
            t2.push((prev, count + 1));
        } else {
            t2.push((prev, count));
            t2.push((t_i, 1));
        }
    }

    if s2.len() != t2.len() {
        println!("No");
        return;
    }

    for ((s_i, s_c), (t_i, t_c)) in s2.iter().copied().zip(t2.iter().copied()) {
        if s_i != t_i {
            println!("No");
            return;
        }

        if s_c == t_c {
            continue;
        }

        if s_c < t_c && s_c >= 2 {
            continue;
        }

        println!("No");
        return;
    }

    println!("Yes");
}
