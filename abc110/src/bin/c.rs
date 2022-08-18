use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ans = true;
    let mut m_s = HashMap::new();
    let mut m_t = HashMap::new();
    for (s_i, t_i) in s.iter().copied().zip(t.iter().copied()) {
        match (m_s.get(&s_i), m_t.get(&t_i)) {
            (None, None) => {
                let j = m_s.len();
                m_s.insert(s_i, j);
                m_t.insert(t_i, j);
            }
            (None, Some(_)) | (Some(_), None) => {
                ans = false;
                break;
            }
            (Some(j1), Some(j2)) => {
                if j1 != j2 {
                    ans = false;
                    break;
                }
            }
        }
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
