use proconio::{input, marker::Usize1};

fn dfs(len: usize, ln: &mut Vec<Vec<usize>>, level: usize, cur: &mut Vec<usize>) {
    if ln.len() > len {
        return;
    }

    if cur.len() == level {
        ln.push(cur.clone());
        return;
    }

    match cur.last() {
        None => {
            for i in 1..=9 {
                cur.push(i);
                dfs(len, ln, level, cur);
                cur.pop();
            }
        }
        Some(last) => {
            for i in last.saturating_sub(1)..=(last + 1).min(9) {
                cur.push(i);
                dfs(len, ln, level, cur);
                cur.pop();
            }
        }
    }
}

fn main() {
    input! {
        k: Usize1,
    };

    let mut ln = vec![];
    for level in 1.. {
        dfs(k + 1, &mut ln, level, &mut vec![]);
        if ln.len() > k + 1 {
            break;
        }
    }
    let ans = ln[k]
        .iter()
        .copied()
        .map(|i| (i as u8 + b'0') as char)
        .collect::<String>();
    println!("{}", ans);
}
