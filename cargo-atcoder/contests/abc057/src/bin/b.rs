use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(i64, i64); n],
        cd: [(i64, i64); m],
    };
    let mut ans = vec![m; n];
    for (i, (a, b)) in ab.iter().copied().enumerate() {
        let mut min = 1_i64 << 60;
        for (j, (c, d)) in cd.iter().copied().enumerate() {
            let dist = (a - c).abs() + (b - d).abs();
            if dist < min {
                min = dist;
                ans[i] = j;
            }
        }
    }

    for a in ans {
        println!("{}", a + 1);
    }
}
