use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    };

    for &(a_i, b_i) in ab.iter() {
        let mut min_dist = 1_000_000_000_000_000_i64;
        let mut x = 0;
        for (i, &(c_i, d_i)) in cd.iter().enumerate() {
            let dist = (a_i - c_i).abs() + (b_i - d_i).abs();
            if dist < min_dist {
                min_dist = dist;
                x = i + 1;
            }
        }
        println!("{}", x);
    }
}
