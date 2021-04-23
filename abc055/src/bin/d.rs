use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn f(s: &Vec<char>, z: bool, o: bool) -> Option<Vec<bool>> {
    let n = s.len();
    let mut d: Vec<Option<bool>> = vec![None; n];
    d[0] = Some(z);
    d[1] = Some(o);
    let mut q = VecDeque::new();
    q.push_back(0);
    q.push_back(1);
    let f1 = |l: usize, r: usize, d: &mut Vec<Option<bool>>, same: bool| match (d[l], d[r]) {
        (None, None) => unreachable!(),
        (None, Some(v_r)) => {
            d[l] = Some(if same { v_r } else { !v_r });
            Some(Some(l))
        }
        (Some(v_l), None) => {
            d[r] = Some(if same { v_l } else { !v_l });
            Some(Some(r))
        }
        (Some(v_l), Some(v_r)) => {
            if (same && v_l != v_r) || (!same && v_l == v_r) {
                None
            } else {
                Some(None)
            }
        }
    };
    while let Some(i) = q.pop_front() {
        let l = (n + i - 1) % n;
        let r = (i + 1) % n;
        let sheep = d[i].unwrap();
        match f1(l, r, &mut d, s[i] == if sheep { 'o' } else { 'x' }) {
            None => return None,
            Some(Some(next)) => q.push_back(next),
            Some(None) => {}
        }
    }
    Some(d.iter().map(|d_i| d_i.unwrap()).collect())
}

fn main() {
    input! {
        _: usize,
        s: Chars,
    };
    for bits in 0..1 << 2 {
        // false: W, true: S
        let sw = (0..2).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        match f(&s, sw[0], sw[1]) {
            Some(d) => {
                for d_i in d {
                    print!("{}", if d_i { "S" } else { "W" });
                }
                println!();
                return;
            }
            None => {}
        }
    }
    println!("-1");
}
