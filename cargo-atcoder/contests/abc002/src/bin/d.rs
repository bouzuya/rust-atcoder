use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyv: [(Usize1, Usize1); m],
    };
    let mut tvv = vec![vec![]; n];
    for (x, y) in xyv {
        tvv[x].push(y);
        tvv[y].push(x);
    }
    let ans = tvv
        .iter()
        .map(|tv| {
            let mut max_count = 0;
            for bits in 0..1_usize << tv.len() {
                let mut uv = vec![];
                for i in 0..tv.len() {
                    if (bits >> i) & 1 != 0 {
                        uv.push(tv[i]);
                    }
                }
                if uv
                    .iter()
                    .all(|&i| uv.iter().all(|&j| j == i || tvv[i].contains(&j)))
                {
                    max_count = std::cmp::max(max_count, uv.len());
                }
            }
            max_count + 1
        })
        .max()
        .unwrap();
    println!("{}", ans);
}
