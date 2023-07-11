use proconio::input;

fn main() {
    input! {
        h: [usize; 3],
        w: [usize; 3],
    };
    let mut ans = 0_usize;
    for v11 in 1..=28 {
        for v12 in 1..=28 {
            if v11 + v12 >= h[0] {
                continue;
            }
            let v13 = h[0] - (v11 + v12);
            for v21 in 1..=28 {
                if v11 + v21 >= w[0] {
                    continue;
                }
                let v31 = w[0] - (v11 + v21);
                for v22 in 1..=28 {
                    if v21 + v22 >= h[1] {
                        continue;
                    }
                    let v23 = h[1] - (v21 + v22);
                    if v12 + v22 >= w[1] {
                        continue;
                    }
                    let v32 = w[1] - (v12 + v22);

                    if v31 + v32 >= h[2] {
                        continue;
                    }
                    let v33 = h[2] - (v31 + v32);
                    if v13 + v23 + v33 == w[2] {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
