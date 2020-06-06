use proconio::input;

fn main() {
    input! {
        n: usize,
        wh: [(i64, i64); n],
    };
    let mut iwh = wh
        .iter()
        .enumerate()
        .map(|(i, &(w, h))| (i, w, h))
        .collect::<Vec<_>>();
    iwh.sort_by_key(|&(_, w, _)| w);
    let mut iw = iwh
        .iter()
        .scan((0, 0), |acc, &(i, w, _)| {
            if acc.0 < w {
                acc.0 = w;
                acc.1 += 1;
            }
            Some((i, acc.1))
        })
        .collect::<Vec<_>>();
    iw.sort();
    iwh.sort_by_key(|&(_, _, h)| h);
    let mut ih = iwh
        .iter()
        .scan((0, 0), |acc, &(i, _, h)| {
            if acc.0 < h {
                acc.0 = h;
                acc.1 += 1;
            }
            Some((i, acc.1))
        })
        .collect::<Vec<_>>();
    ih.sort();
    let mut cwh = iw
        .iter()
        .zip(ih.iter())
        .map(|(&(_, w), &(_, h))| (w, h))
        .collect::<Vec<_>>();
    cwh.sort_by_key(|&(w, h)| std::cmp::Reverse((w, h)));
    let mut c = 0;
    let mut wh_p = (1_000_000, 1_000_000);
    for &(w, h) in cwh.iter() {
        if w < wh_p.0 && h < wh_p.1 {
            c += 1;
            wh_p = (w, h);
        }
    }
    let ans = c;
    println!("{}", ans);
}
