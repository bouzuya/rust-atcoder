use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let n = s.len();
    let m = t.len();

    let mut ok = s
        .iter()
        .copied()
        .take(m)
        .zip(t.iter().copied())
        .filter(|(s_i, t_i)| s_i == &'?' || t_i == &'?' || s_i == t_i)
        .count();
    let mut ans = vec![false; m + 1];
    ans[m] = ok == m;
    let mut j = n - 1;
    for i in (0..m).rev() {
        let old_ok = s[i] == '?' || t[i] == '?' || s[i] == t[i];
        if old_ok {
            ok -= 1;
        }
        let new_ok = s[j] == '?' || t[i] == '?' || s[j] == t[i];
        if new_ok {
            ok += 1;
        }
        ans[i] = ok == m;
        j -= 1;
    }

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
