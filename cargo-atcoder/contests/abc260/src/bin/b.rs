use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let mut abi = a
        .into_iter()
        .zip(b.into_iter())
        .enumerate()
        .map(|(i, (a_i, b_i))| (a_i, b_i, i))
        .collect::<Vec<(usize, usize, usize)>>();

    let mut used = vec![false; n];
    let mut ans = vec![];

    let mut count = 0;
    abi.sort_by_key(|(a, b, i)| (Reverse(*a), *i));
    for (_, _, i) in abi.iter().copied() {
        if count >= x {
            break;
        }
        if used[i] {
            continue;
        }
        count += 1;
        used[i] = true;
        ans.push(i);
    }

    let mut count = 0;
    abi.sort_by_key(|(a, b, i)| (Reverse(*b), *i));
    for (_, _, i) in abi.iter().copied() {
        if count >= y {
            break;
        }
        if used[i] {
            continue;
        }
        count += 1;
        used[i] = true;
        ans.push(i);
    }

    let mut count = 0;
    abi.sort_by_key(|(a, b, i)| (Reverse(*a + *b), *i));
    for (_, _, i) in abi.iter().copied() {
        if count >= z {
            break;
        }
        if used[i] {
            continue;
        }
        count += 1;
        used[i] = true;
        ans.push(i);
    }

    ans.sort();
    for a in ans {
        println!("{}", a + 1);
    }
}
