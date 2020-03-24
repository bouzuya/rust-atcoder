use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        h: usize,
        w: usize,
        k: usize,
        svv: [Bytes; h],
    };

    let inf = h * w;
    let mut ans = inf;
    for hdbits in 0..(1 << (h - 1)) {
        let mut rgv = vec![0; h]; // gv[y] = row_group_index
        let mut hdc = 0;
        for y in 0..h {
            rgv[y] = hdc;
            if ((hdbits >> y) & 1) == 1 {
                hdc += 1;
            }
        }

        let mut rgcv = vec![vec![0_usize; w]; hdc + 1];
        for y in 0..h {
            for x in 0..w {
                rgcv[rgv[y]][x] += if svv[y][x] == b'1' { 1 } else { 0 };
            }
        }

        let mut vdc = 0;
        let mut gcv = vec![0_usize; rgcv.len()];
        for x in 0..w {
            if (0..rgcv.len()).any(|g| rgcv[g][x] > k) {
                vdc = inf;
                break;
            }
            for g in 0..rgcv.len() {
                gcv[g] += rgcv[g][x];
            }
            if (0..rgcv.len()).any(|g| gcv[g] > k) {
                vdc += 1;
                for g in 0..rgcv.len() {
                    gcv[g] = rgcv[g][x];
                }
            }
        }
        ans = std::cmp::min(ans, hdc + vdc);
    }
    println!("{}", ans);
}
