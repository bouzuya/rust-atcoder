use proconio::{input, marker::Chars};

fn f(q: &Vec<(usize, usize)>) -> bool {
    for i in 0..q.len() {
        for j in i + 1..q.len() {
            if (q[i].0 == q[j].0)
                || (q[i].1 == q[j].1)
                || ((q[i].0 + 1) + (q[i].1 + 1) == (q[j].0 + 1) + (q[j].1 + 1))
                || ((8 + q[i].0 - q[i].1) == (8 + q[j].0 - q[j].1))
            {
                return false;
            }
        }
    }
    true
}

fn dfs(q: &mut Vec<(usize, usize)>) -> bool {
    if q.len() == 8 {
        return f(q);
    }
    let mut r = 0;
    'outer: for i in 0..8 {
        for &(q_i, _) in q.iter() {
            if q_i == i {
                continue 'outer;
            }
        }
        r = i;
        break;
    }

    for j in 0..8 {
        q.push((r, j));
        if dfs(q) {
            return true;
        }
        q.pop();
    }

    false
}

fn main() {
    input! {
        c: [Chars; 8]
    };
    let mut q = vec![];
    for i in 0..8 {
        for j in 0..8 {
            if c[i][j] == 'Q' {
                q.push((i, j));
            }
        }
    }
    if !f(&q) {
        println!("No Answer");
        return;
    }

    if !dfs(&mut q) {
        println!("No Answer");
        return;
    }

    for i in 0..8 {
        for j in 0..8 {
            let mut p = false;
            for &(r, c) in q.iter() {
                if r == i && c == j {
                    p = true;
                    break;
                }
            }
            print!("{}", if p { "Q" } else { "." });
        }
        println!();
    }
}
