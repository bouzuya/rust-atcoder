use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[usize; 6]; n],
    };

    let o = {
        let mut x = vec![];
        for (i, a_i) in a.iter().enumerate() {
            for &a_ij in a_i.iter() {
                x.push((a_ij, i));
            }
        }
        x.sort_by_key(|&p| std::cmp::Reverse(p));
        x.iter().map(|&(_, i)| i).collect::<Vec<usize>>()
    };

    let mut d = vec![0_f64; n];
    let mut m = 0_f64;
    for &i in o.iter() {
        d[i] += (m + 1_f64) / 6_f64;
        m = m.max(d[i]);
    }
    let ans = m + 1_f64;
    println!("{}", ans);
}
