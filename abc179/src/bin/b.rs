use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(i64, i64); n],
    };
    let mut p = vec![(0, 0), (0, 0)];
    p[0] = d[0];
    p[1] = d[1];
    for &(d_i_1, d_i_2) in d.iter().skip(2) {
        if p[0].0 == p[0].1 && p[1].0 == p[1].1 && d_i_1 == d_i_2 {
            println!("Yes");
            return;
        }
        p[0] = p[1];
        p[1] = (d_i_1, d_i_2);
    }
    println!("No");
}
