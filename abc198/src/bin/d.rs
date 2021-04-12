use proconio::input;
use proconio::marker::Chars;
use superslice::Ext;

fn is_ok(s: &Vec<Vec<usize>>, d: &Vec<i64>) -> bool {
    if d[s[0][0]] == 0 || d[s[1][0]] == 0 || d[s[2][0]] == 0 {
        return false;
    }
    let l = s.iter().map(|s_i| s_i.len()).collect::<Vec<usize>>();
    let mut dc = 0;
    for i in 0..10 {
        let d0 = if i >= l[0] { 0 } else { d[s[0][l[0] - 1 - i]] };
        let d1 = if i >= l[1] { 0 } else { d[s[1][l[1] - 1 - i]] };
        let d2 = if i >= l[2] { 0 } else { d[s[2][l[2] - 1 - i]] };
        if (dc + d0 + d1) % 10 != d2 {
            return false;
        }
        dc = (dc + d0 + d1) / 10;
    }
    true
}

fn print(s: &Vec<Vec<usize>>, d: &Vec<i64>) {
    for s_i in s.iter() {
        for &j in s_i.iter() {
            print!("{}", d[j]);
        }
        println!();
    }
}

fn main() {
    input! {
        s: [Chars; 3],
    };
    let mut t: Vec<char> = vec![];
    for s_i in s.iter() {
        for &s_ij in s_i.iter() {
            if !t.contains(&s_ij) {
                t.push(s_ij);
            }
        }
    }
    if t.len() > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let s = {
        let mut s2 = vec![vec![]; 3];
        for (i, s_i) in s.iter().enumerate() {
            for &s_ij in s_i.iter() {
                for (k, &t_k) in t.iter().enumerate() {
                    if t_k == s_ij {
                        s2[i].push(k);
                    }
                }
            }
        }
        s2
    };
    let mut ds = (0..=9).collect::<Vec<i64>>();
    loop {
        if is_ok(&s, &ds) {
            print(&s, &ds);
            return;
        }
        if !ds.next_permutation() {
            break;
        }
    }
    println!("UNSOLVABLE");
}
