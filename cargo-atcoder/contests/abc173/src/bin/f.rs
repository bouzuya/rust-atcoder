use proconio::input;

fn main() {
    input! {
        n: i64,
        uv: [(i64, i64); n - 1],
    };

    // \sum_{l=1}^{n} \sum_{r=l}^{n} r - l + 1
    let mut c_v = 0_i64;
    for l in 1..=n {
        c_v += l * (n - l + 1)
    }

    let mut c_e = 0_i64;
    for &(u, v) in uv.iter() {
        let (l, r) = if u < v { (u, v) } else { (v, u) };
        c_e += l * (n - r + 1)
    }

    let ans = c_v - c_e;
    println!("{}", ans);
}
