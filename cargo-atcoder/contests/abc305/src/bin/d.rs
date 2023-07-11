use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    };

    // let map = a
    //     .iter()
    //     .copied()
    //     .collect::<BTreeSet<_>>()
    //     .into_iter()
    //     .enumerate()
    //     .fold(BTreeMap::new(), |mut m, (i, k)| {
    //         m.insert(k, i);
    //         m
    //     });
    // let mut c = vec![0_usize; map.len()];
    // for (&k, &v) in map.iter() {
    //     if v == 0 {
    //         continue;
    //     }
    //     if v % 2 == 0 {

    //     }
    //     // c[v] =

    // }
    let mut c = vec![0_usize; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        if i == 0 {
            continue;
        }
        if i % 2 == 0 {
            c[i] = c[i - 2] + a_i - a[i - 1];
        } else {
            c[i] = c[i - 1];
        }
    }
    // println!("{:?}", a);
    // println!("{:?}", c);

    for (l, r) in lr {
        let index_l = a.lower_bound(&l);
        let c_l = if index_l % 2 != 0 {
            c[index_l]
        } else {
            c[index_l] - (a[index_l] - l)
        };
        let index_r = a.lower_bound(&r);
        let c_r = if index_r == n {
            c[n - 1] - (a[n - 1] - r)
        } else if index_r % 2 == 0 {
            c[index_r] - (a[index_r] - r)
        } else {
            c[index_r - 1]
        };
        let ans = c_r - c_l;
        println!("{}", ans);
    }
}
