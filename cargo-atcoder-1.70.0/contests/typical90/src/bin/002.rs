use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = BTreeSet::new();
    for bits in 0..1 << n {
        let cs = (0..n)
            .map(|i| if ((bits >> i) & 1) == 1 { ')' } else { '(' })
            .collect::<Vec<char>>();
        let mut ok = true;
        let mut stack = vec![];
        for c in cs.iter().copied() {
            match c {
                '(' => stack.push(c),
                ')' => match stack.pop() {
                    None => {
                        ok = false;
                        break;
                    }
                    Some(p) => {
                        if p != '(' {
                            ok = false;
                            break;
                        }
                    }
                },
                _ => unreachable!(),
            }
        }
        ok &= stack.is_empty();
        if ok {
            ans.insert(cs.into_iter().collect::<String>());
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
