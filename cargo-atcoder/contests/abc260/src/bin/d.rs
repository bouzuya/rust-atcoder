use std::collections::{BTreeMap, HashMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    };
    let mut turn = HashMap::new();
    let mut stacks = vec![];
    let mut top = BTreeMap::new();

    for (i, p_i) in p.iter().copied().enumerate() {
        let updated_stack_index = match top.range(p_i..=n).next() {
            None => {
                let stack_index = stacks.len();
                top.insert(p_i, stack_index);
                stacks.push(vec![p_i]);
                stack_index
            }
            Some((&old_p_i, &stack_index)) => {
                top.remove(&old_p_i);
                stacks[stack_index].push(p_i);
                top.insert(p_i, stack_index);
                stack_index
            }
        };
        if stacks[updated_stack_index].len() >= k {
            for s in stacks[updated_stack_index].iter().copied() {
                turn.insert(s, i);
                top.remove(&s);
            }
        }
    }

    for i in 0..n {
        let ans = match turn.get(&i) {
            Some(d) => (d + 1) as i64,
            None => -1_i64,
        };
        println!("{}", ans)
    }
}
