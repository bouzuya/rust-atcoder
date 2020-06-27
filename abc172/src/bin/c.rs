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
        m: usize,
        k: i64,
        a: [i64; n],
        b: [i64; m],
    };
    let ca = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, a_i| {
            *acc += a_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let cb = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, b_i| {
            *acc += b_i;
            Some(*acc)
        }))
        .collect::<Vec<_>>();

    let mut ans = 0;
    let mut max_j = cb.len() - 1;
    for (i, &ca_i) in ca.iter().enumerate() {
        for (j, &cb_i) in cb[0..=max_j].iter().enumerate().rev() {
            if ca_i + cb_i <= k {
                chmax!(ans, i + j);
                max_j = j;
                break;
            }
        }
    }

    println!("{}", ans);
}
