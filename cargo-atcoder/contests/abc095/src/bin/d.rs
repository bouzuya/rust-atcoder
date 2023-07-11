use proconio::input;

fn main() {
    input! {
        n: usize,
        c: isize,
        xvv: [(isize, isize); n],
    };
    // ->
    let mut rcv = vec![(0_isize, 0_isize); n];
    rcv[0].0 = xvv[0].0;
    rcv[0].1 = xvv[0].1;
    for i in 1..n {
        rcv[i].0 = xvv[i].0;
        rcv[i].1 = rcv[i - 1].1 + xvv[i].1;
    }
    // println!("{:?}", rcv);

    // <-
    let mut lcv = vec![(0_isize, 0_isize); n];
    lcv[n - 1].0 = c - xvv[n - 1].0;
    lcv[n - 1].1 = xvv[n - 1].1;
    for i in (0..n - 1).rev() {
        lcv[i].0 = c - xvv[i].0;
        lcv[i].1 = lcv[i + 1].1 + xvv[i].1;
    }
    // println!("{:?}", lcv);

    // ->
    let mut p1v = vec![(0_isize, 0_isize); n];
    p1v[0] = (rcv[0].1 - rcv[0].0, rcv[0].1 - 2 * rcv[0].0);
    for i in 1..n {
        let (rx, rv) = rcv[i];
        p1v[i] = (
            std::cmp::max(p1v[i - 1].0, rv - rx),
            std::cmp::max(p1v[i - 1].1, rv - 2 * rx),
        );
    }
    let mut p2v = vec![(0_isize, 0_isize); n];
    p2v[n - 1] = (lcv[n - 1].1 - lcv[n - 1].0, lcv[n - 1].1 - 2 * lcv[n - 1].0);
    for i in 0..n - 1 {
        let (lx, lv) = lcv[i];
        p2v[i] = (
            std::cmp::max(p2v[i + 1].0, lv - lx),
            std::cmp::max(p2v[i + 1].1, lv - 2 * lx),
        );
    }
    // println!("{:?}", p1v);
    // println!("{:?}", p2v);

    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, std::cmp::max(p1v[i].0, p2v[i].0));
    }
    for i in 0..n - 1 {
        ans = std::cmp::max(ans, p1v[i].0 + p2v[i + 1].1);
        ans = std::cmp::max(ans, p1v[i].1 + p2v[i + 1].0);
    }
    println!("{}", ans);
}
