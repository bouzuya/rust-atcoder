use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut s = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [Usize1; c],
        }
        s.push(a);
    }

    let mut count = 0_usize;
    for bits in 1..1 << m {
        let mut set = vec![];
        for i in 0..m {
            if (bits >> i) & 1 == 1 {
                set.push(s[i].clone());
            }
        }
        let mut ok = true;
        for i in 0..n {
            let mut contains = false;
            for set_i in set.iter() {
                if set_i.contains(&i) {
                    contains = true;
                    break;
                }
            }
            if !contains {
                ok = false;
                break;
            }
        }

        if ok {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
