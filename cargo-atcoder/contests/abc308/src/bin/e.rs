use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: Chars,
    };

    let mut map_x = vec![0_usize; 3];
    for (s_i, a_i) in s.iter().copied().zip(a.iter().copied()) {
        if s_i != 'X' {
            continue;
        }
        map_x[a_i] += 1;
    }

    let mut map_m = vec![0_usize; 3];

    let mut sum = 0_usize;
    for (s_i, a_i) in s.iter().copied().zip(a.iter().copied()) {
        match s_i {
            'M' => map_m[a_i] += 1,
            'E' => {}
            'X' => map_x[a_i] -= 1,
            _ => unreachable!(),
        }

        if s_i != 'E' {
            continue;
        }

        let e = a_i;
        for m in 0..3 {
            for x in 0..3 {
                let mut set = HashSet::new();
                set.insert(m);
                set.insert(e);
                set.insert(x);
                let mut mex = 3;
                for j in 0..3 {
                    if !set.contains(&j) {
                        mex = j;
                        break;
                    }
                }
                sum += map_m[m] * map_x[x] * mex;
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}
