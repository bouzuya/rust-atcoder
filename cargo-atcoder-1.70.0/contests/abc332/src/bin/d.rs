use ac_library::FenwickTree;
use proconio::input;
use superslice::Ext;

fn inversion_number(a: &[usize]) -> usize {
    let mut count = 0;
    let mut ft = FenwickTree::new(a.len(), 0);
    for (i, &a_i) in a.iter().enumerate() {
        ft.add(a_i, 1);
        count += i - ft.accum(a_i);
    }
    count
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
        b: [[usize; w]; h],
    };

    let inf = 1_000_000_000_usize;
    let mut min = inf;
    let mut is = (0..h).collect::<Vec<usize>>();
    loop {
        let mut js = (0..w).collect::<Vec<usize>>();

        loop {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    if a[is[i]][js[j]] != b[i][j] {
                        ok = false;
                    }
                }
            }
            if ok {
                min = min.min(inversion_number(&is) + inversion_number(&js));
            }
            if !js.next_permutation() {
                break;
            }
        }

        if !is.next_permutation() {
            break;
        }
    }

    if min == inf {
        println!("-1");
    } else {
        let ans = min;
        println!("{}", ans);
    }
}
