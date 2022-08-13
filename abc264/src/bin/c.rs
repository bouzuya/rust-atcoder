use proconio::input;

fn main() {
    input! {
        h_1: usize,
        w_1: usize,
        a: [[usize; w_1]; h_1],
        h_2: usize,
        w_2: usize,
        b: [[usize; w_2]; h_2],
    };

    for bits in 0..1 << h_1 {
        let mut is = vec![false; h_1];
        for i in 0..h_1 {
            is[i] = (bits >> i) & 1 == 1;
        }
        for bits in 0..1 << w_1 {
            let mut js = vec![false; w_1];
            for j in 0..w_1 {
                js[j] = (bits >> j) & 1 == 1;
            }

            let mut a2 = vec![];
            for k in 0..h_1 {
                if !is[k] {
                    continue;
                }
                let mut a2_i = vec![];
                for l in 0..w_1 {
                    if !js[l] {
                        continue;
                    }
                    a2_i.push(a[k][l]);
                }
                a2.push(a2_i);
            }

            if a2 == b {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
