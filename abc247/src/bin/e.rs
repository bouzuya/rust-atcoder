use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        a: [usize; n],
    };
    // x : max
    // y : min
    let mut pos_x = vec![];
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == x {
            pos_x.push(i);
        }
    }
    let mut pos_y = vec![];
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == y {
            pos_y.push(i);
        }
    }
    let mut pos_n = vec![];
    for (i, a_i) in a.iter().copied().enumerate() {
        if !(y..=x).contains(&a_i) {
            pos_n.push(i);
        }
    }

    // println!("pos_x = {:?}", pos_x);
    // println!("pos_y = {:?}", pos_y);
    // println!("pos_n = {:?}", pos_n);
    let mut count = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        let pl = *pos_n.get(pos_n.lower_bound(&(i + 1))).unwrap_or(&n);
        let pos = match (a_i == x, a_i == y) {
            (true, true) => {
                match (
                    pos_y.get(pos_y.lower_bound(&(i + 1))),
                    pos_x.get(pos_x.lower_bound(&(i + 1))),
                ) {
                    (Some(min_y), Some(min_x)) => Some(min_y.min(min_x)),
                    (Some(_), None) => None,
                    (None, Some(_)) => None,
                    (None, None) => None,
                }
            }
            (true, false) => {
                // 直近の MIN から→を数える
                pos_y.get(pos_y.lower_bound(&(i + 1)))
            }
            (false, true) => {
                // 直近の MAX から→を数える
                pos_x.get(pos_x.lower_bound(&(i + 1)))
            }
            (false, false) => {
                if (y..=x).contains(&a_i) {
                    // 直近の MIN と直近の MAX の遠い側から→を数える
                    match (
                        pos_y.get(pos_y.lower_bound(&(i + 1))),
                        pos_x.get(pos_x.lower_bound(&(i + 1))),
                    ) {
                        (Some(min_y), Some(min_x)) => Some(min_y.max(min_x)),
                        (Some(_), None) => None,
                        (None, Some(_)) => None,
                        (None, None) => None,
                    }
                } else {
                    None
                }
            }
        };
        let p = *pos.unwrap_or(&n);
        let d = pl.saturating_sub(p);
        count += d;

        // println!(
        //     "{:?} A[{}] = {}, count = {}, p = {:?}, pl = {} d = {}",
        //     (a_i == x, a_i == y),
        //     i,
        //     a_i,
        //     count,
        //     p,
        //     pl,
        //     d
        // );
    }
    let ans = count + if x == y { pos_x.len() } else { 0 };
    println!("{}", ans);
}
