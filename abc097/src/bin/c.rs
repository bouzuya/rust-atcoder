use std::collections::BTreeSet;

use proconio::input;

fn dfs(chars: &Vec<char>, l: usize, b: &mut String, res: &mut Vec<String>) {
    if b.len() == l {
        res.push(b.clone());
        return;
    }
    for &c in chars {
        b.push(c);
        dfs(chars, l, b, res);
        b.pop();
    }
}

fn main() {
    input! {
        s: String,
        k: usize,
    };
    let chars = s
        .chars()
        .collect::<BTreeSet<char>>()
        .iter()
        .cloned()
        .collect::<Vec<char>>();
    let mut t = vec![];
    for l in 1..=k {
        dfs(&chars, l, &mut String::new(), &mut t);
    }
    t.sort();

    let mut c = 0;
    for t_i in t.iter() {
        if s.contains(t_i.as_str()) {
            c += 1;
            if c == k {
                println!("{}", t_i);
                break;
            }
        }
    }
}
