use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let groups = {
        let mut groups = vec![];
        // (r_l, l_l, l_r)
        let mut cur = (0, 0, 0);
        let mut is_r = true;
        for (i, s_i) in s.iter().copied().enumerate() {
            match s_i {
                'R' => {
                    if !is_r {
                        is_r = true;
                        cur.2 = i - 1;
                        groups.push(cur);
                        cur = (i, i, 0);
                    }
                }
                'L' => {
                    if is_r {
                        is_r = false;
                        cur.1 = i;
                    } else {
                        cur.2 = i;
                    }
                }
                _ => unreachable!(),
            }
        }
        cur.2 = s.len() - 1;
        groups.push(cur);
        groups
    };
    let mut ans = vec![0; s.len()];
    for (r_l, l_l, l_r) in groups {
        let index_r = l_l - 1;
        let index_l = l_l;
        let count_r = l_l - r_l - 1;
        let count_l = l_r - l_l + 1 - 1;
        ans[index_r] = 1 + count_r / 2 + count_l / 2 + count_l % 2;
        ans[index_l] = 1 + count_r / 2 + count_l / 2 + count_r % 2;
    }

    for a in ans {
        println!("{}", a);
    }
}
