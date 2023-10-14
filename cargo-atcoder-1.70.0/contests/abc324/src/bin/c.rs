use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        s: [Chars; n],
    };
    let mut ans = vec![];
    for (i, s_i) in s.iter().enumerate() {
        if s_i.len() == t.len() {
            // p1
            if s_i == &t {
                ans.push(i);
                continue;
            }

            // p4
            if s_i
                .iter()
                .copied()
                .zip(t.iter().copied())
                .filter(|(s_ij, t_j)| s_ij != t_j)
                .count()
                <= 1
            {
                ans.push(i);
                continue;
            }
        } else if s_i.len() == t.len() + 1 {
            // p2
            let mut count = 0_usize;
            let mut it1 = s_i.iter();
            let mut it2 = t.iter();
            loop {
                match (it1.next(), it2.next()) {
                    (None, None) => {
                        ans.push(i);
                        break;
                    }
                    (None, Some(_)) => {
                        break;
                    }
                    (Some(_), None) => {
                        if count == 0 {
                            ans.push(i);
                        }
                        break;
                    }
                    (Some(a), Some(b)) => {
                        if a == b {
                            continue;
                        }
                        count += 1;
                        if count > 1 {
                            break;
                        }
                        if Some(b) != it1.next() {
                            break;
                        }
                    }
                }
            }
        } else if s_i.len() + 1 == t.len() {
            // p3
            let mut count = 0_usize;
            let mut it1 = s_i.iter();
            let mut it2 = t.iter();
            loop {
                match (it1.next(), it2.next()) {
                    (None, None) => {
                        ans.push(i);
                        break;
                    }
                    (None, Some(_)) => {
                        if count == 0 {
                            ans.push(i);
                        }
                        break;
                    }
                    (Some(_), None) => {
                        break;
                    }
                    (Some(a), Some(b)) => {
                        if a == b {
                            continue;
                        }
                        count += 1;
                        if count > 1 {
                            break;
                        }
                        if Some(a) != it2.next() {
                            break;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans.len());
    for a in ans {
        println!("{}", a + 1);
    }
}
