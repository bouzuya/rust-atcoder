use proconio::input;

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
        n: usize,
        k: usize,
        mut lr: [(usize, usize); n],
    };
    let lr_bak = lr.clone();

    lr.sort_by_key(|&(l, r)| (r, l));
    let mut count = 0_usize;
    let mut count_l = vec![0_usize; 86_400 + k + 1];
    let mut cur = 0_usize;
    for (l, r) in lr.iter().copied() {
        if cur <= l {
            count += 1;
            count_l[r + k] = count; // ?
            cur = r + k;
        }
    }

    lr.sort_by_key(|&(l, r)| (l, r));
    let mut count = 0_usize;
    let mut count_r = vec![0_usize; 86_400 + k + 1];
    let mut cur = 1_usize << 60;
    for (l, r) in lr.iter().copied().rev() {
        if cur >= r + k {
            count += 1;
            count_r[l] = count;
            cur = l;
        }
    }

    for i in 0..86_400 + k {
        chmax!(count_l[i + 1], count_l[i]);
    }
    for i in (1..=86_400 + k).rev() {
        chmax!(count_r[i - 1], count_r[i]);
    }

    for (l, r) in lr_bak.iter().copied() {
        println!("{}", count_l[l] + 1 + count_r[r + k]);
    }
}
