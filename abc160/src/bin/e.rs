use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut av: [usize; a],
        mut bv: [usize; b],
        mut cv: [usize; c],
    };
    av.sort_by(|a, b| b.cmp(&a));
    bv.sort_by(|a, b| b.cmp(&a));
    cv.sort_by(|a, b| b.cmp(&a));

    let mut vv = vec![0_usize; x + y + x + y];
    for i in 0..x {
        vv[i] = av[i];
    }
    for i in 0..y {
        vv[x + i] = bv[i];
    }
    for i in 0..std::cmp::min(cv.len(), x + y) {
        vv[x + y + i] = cv[i];
    }
    vv.sort_by(|a, b| b.cmp(&a));

    let mut ans = 0;
    for i in 0..x + y {
        ans += vv[i];
    }
    println!("{}", ans);
}
