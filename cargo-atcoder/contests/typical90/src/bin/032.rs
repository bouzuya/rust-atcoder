use proconio::input;
use proconio::marker::Usize1;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [[u64; n]; n],
        m: usize,
        xy: [(Usize1, Usize1); m],
    };
    let mut is = (0..n).collect::<Vec<usize>>();
    let mut ans = None;
    loop {
        let mut ok = true;
        let mut p = is[0];
        for &i in is.iter().skip(1) {
            for &(x_i, y_i) in xy.iter() {
                if (x_i == p && y_i == i) || (x_i == i && y_i == p) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
            p = i;
        }
        if ok {
            let v = is.iter().enumerate().map(|(j, &i)| a[i][j]).sum::<u64>();
            ans = match ans {
                None => Some(v),
                Some(p) => {
                    if v < p {
                        Some(v)
                    } else {
                        Some(p)
                    }
                }
            }
        }
        if !is.next_permutation() {
            break;
        }
    }
    println!("{}", ans.map(|x| x as i64).unwrap_or(-1));
}
