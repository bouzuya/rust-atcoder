use proconio::input;
use proconio::marker::Usize1;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        _: usize,
        _: usize,
        m: usize,
        hw: [(Usize1, Usize1); m],
    };

    let mut map_h = std::collections::BTreeMap::new();
    let mut map_w = std::collections::BTreeMap::new();
    for (i, &(h_i, w_i)) in hw.iter().enumerate() {
        map_h.entry(h_i).or_insert(vec![]).push(i);
        map_w.entry(w_i).or_insert(vec![]).push(i);
    }

    let ans1 = {
        let mut max_h = 0;
        let mut max_h_i = 0;
        for (h_i, is) in map_h.iter() {
            if chmax!(max_h, is.len()) {
                max_h_i = *h_i;
            }
        }
        let mut max_w = 0;
        for (_, is) in map_w.iter() {
            let len = is.iter().filter(|&&i| hw[i].0 != max_h_i).count();
            chmax!(max_w, len);
        }
        max_h + max_w
    };
    let ans2 = {
        let mut max_w = 0;
        let mut max_w_i = 0;
        for (w_i, is) in map_w.iter() {
            if chmax!(max_w, is.len()) {
                max_w_i = *w_i;
            }
        }
        let mut max_h = 0;
        for (_, is) in map_h.iter() {
            let len = is.iter().filter(|&&i| hw[i].1 != max_w_i).count();
            chmax!(max_h, len);
        }
        max_h + max_w
    };

    let ans = std::cmp::max(ans1, ans2);
    println!("{}", ans);
}
