use proconio::input;

fn main() {
    input! {
        n: usize,
        mut v: [f64; n],
    };
    v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    while v.len() != 1 {
        let z = (v[0] + v[1]) / 2_f64;
        v = v.into_iter().skip(2).collect::<Vec<f64>>();
        v.push(z);
        v.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    }
    let ans = v[0];
    println!("{}", ans);
}
