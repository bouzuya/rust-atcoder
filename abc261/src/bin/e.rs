use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        ta: [(usize, usize); n],
    };

    let mut ans = vec![0; n];
    let mask = (1_usize << 63) - 1;
    for i in 0..63 {
        let mut xor = 0;
        let mut x = (c >> i) & 1;
        let mut y = (c >> i) & 1;
        let mut f = false;
        for (j, (t_j, a_j)) in ta.iter().copied().enumerate() {
            let a_j = (a_j >> i) & 1;
            match t_j {
                1 => {
                    // and
                    if a_j == 0 {
                        x = 0;
                        xor = 0;
                        f = true;
                    }
                }
                2 => {
                    // or
                    if a_j == 1 {
                        x = 1;
                        xor = 0;
                        f = true;
                    }
                }
                3 => {
                    // xor
                    if a_j == 1 {
                        xor += 1;
                        xor %= 2;
                    }
                }
                _ => unreachable!(),
            }
            y = if f { x ^ xor } else { y ^ xor };
            ans[j] &= !(mask & (1 << i));
            ans[j] |= y << i;
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
