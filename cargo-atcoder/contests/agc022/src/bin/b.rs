use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let ans = if n == 3 {
        vec![2, 5, 63]
    } else if n == 4 {
        vec![2, 5, 20, 63]
    } else if n == 5 {
        vec![2, 5, 20, 30, 63]
    } else {
        let mut sum = 0_usize;
        let mut ans = BTreeSet::new();
        for k in 0.. {
            let mut v = 0;
            for d in &[2_usize, 3, 4, 6] {
                if ans.len() == n {
                    break;
                }
                v = 6 * k + d;
                ans.insert(v);
                sum += v;
                sum %= 6;
            }
            if ans.len() == n {
                match sum {
                    0 => {}
                    2 => {
                        ans.remove(&(6 + 2));
                        ans.insert(((v / 6) + 1) * 6);
                    }
                    3 => {
                        ans.remove(&(6 + 3));
                        ans.insert(((v / 6) + 1) * 6);
                    }
                    5 => {
                        ans.remove(&(6 + 3));
                        ans.insert((v / 6) * 6 + 4);
                        if ans.len() < n {
                            ans.insert(((v / 6) + 1) * 6 + 4);
                        }
                    }
                    _ => unreachable!(),
                }
                break;
            }
        }
        ans.into_iter().collect::<Vec<usize>>()
    };
    assert!(ans.len() == n);

    for a in ans {
        println!("{}", a);
    }
}
