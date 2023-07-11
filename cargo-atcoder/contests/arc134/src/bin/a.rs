use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        w: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    let mut p = 0;
    for (i, a_i) in a.iter().copied().enumerate() {
        let end_p = if i == 0 { p } else { p + w };
        if end_p < a_i {
            let d = a_i - end_p;
            sum += (d + w - 1) / w;
        }
        p = a_i;
    }
    let end_p = p + w;
    let a_i = l;
    if end_p < a_i {
        let d = a_i - end_p;
        sum += (d + w - 1) / w;
    }
    let ans = sum;
    println!("{}", ans);
}
