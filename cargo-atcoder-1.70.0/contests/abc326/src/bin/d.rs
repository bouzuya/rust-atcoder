use proconio::{input, marker::Chars};
use superslice::Ext;

fn g(ans: &mut [Vec<char>], n: usize, capital_r: &[char], capital_c: &[char], r: usize) -> bool {
    for (capital_r_i, ans_i) in capital_r.iter().copied().zip(ans.iter()).take(r) {
        match ans_i.iter().copied().find(|c| c != &'.') {
            None => return false,
            Some(first) => {
                if first != capital_r_i {
                    return false;
                }
            }
        }
    }

    for (j, capital_c_j) in capital_c.iter().copied().enumerate() {
        match (0..r).map(|i| ans[i][j]).find(|c| c != &'.') {
            None => {
                if r == n {
                    return false;
                }
            }
            Some(first) => {
                if first != capital_c_j {
                    return false;
                }
            }
        }
        let count =
            (0..r)
                .map(|i| ans[i][j])
                .filter(|c| c != &'.')
                .fold(vec![0; 3], |mut count, c| {
                    count[(c as u8 - b'A') as usize] += 1;
                    count
                });
        if r == n {
            if count != vec![1; 3] {
                return false;
            }
        } else if count.iter().any(|c| c > &1) {
            return false;
        }
    }

    true
}

fn f(ans: &mut Vec<Vec<char>>, n: usize, capital_r: &[char], capital_c: &[char], r: usize) -> bool {
    if r == n {
        return g(ans, n, capital_r, capital_c, r);
    }
    for ans_i in ans.iter_mut().skip(r) {
        ans_i.sort();
    }
    loop {
        if g(ans, n, capital_r, capital_c, r + 1) && f(ans, n, capital_r, capital_c, r + 1) {
            return true;
        }
        if !ans[r].next_permutation() {
            break;
        }
    }
    false
}

fn main() {
    input! {
        n: usize,
        capital_r: Chars,
        capital_c: Chars,
    };

    let mut ans = vec![];
    for _ in 0..n {
        let chars = "ABC..".chars().take(n).collect::<Vec<char>>();
        ans.push(chars);
    }
    if f(&mut ans, n, &capital_r, &capital_c, 0) {
        println!("Yes");
        for ans_i in ans.iter() {
            for ans_ij in ans_i.iter() {
                print!("{}", ans_ij);
            }
            println!();
        }
    } else {
        println!("No");
    }
}
