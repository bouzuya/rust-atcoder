use proconio::input;

fn main() {
    input! {
        n: usize,
        t: i64,
        a: i64,
        h: [i64; n],
    };
    let mut min_v = None;
    for (i, &h_i) in h.iter().enumerate() {
        let t_i = t * 1000 - h_i * 6;
        let d_i = (a * 1000 - t_i).abs();
        min_v = Some(match min_v {
            None => (d_i, i),
            Some((d_p, p)) => {
                if d_p < d_i {
                    (d_p, p)
                } else {
                    (d_i, i)
                }
            }
        })
    }
    let ans = min_v.unwrap().1 + 1;
    println!("{}", ans);
}
