use proconio::input;

fn main() {
    input! {
        l: usize,
        b: [usize; l],
    };
    let bit_len = 64;
    let b = {
        let mut c = vec![vec![false; bit_len]; l];
        for i in 0..l {
            for j in 0..bit_len {
                c[i][j] = ((b[i] >> j) & 1) == 1;
            }
        }
        c
    };
    let mut a = vec![vec![false; bit_len]; l];
    for j in 0..bit_len {
        for i in (0..l).take(l - 1) {
            if !b[i][j] {
                a[i + 1][j] = a[i][j];
            } else {
                a[i + 1][j] = !a[i][j];
            }
        }
        match (b[l - 1][j], a[l - 1][j], a[0][j]) {
            (false, false, true)
            | (false, true, false)
            | (true, false, false)
            | (true, true, true) => {
                println!("-1");
                return;
            }
            _ => {}
        }
    }

    let a = {
        let mut c = vec![0_usize; l];
        for j in 0..bit_len {
            for i in 0..l {
                if a[i][j] {
                    c[i] |= 1 << j;
                }
            }
        }
        c
    };

    for a_i in a {
        println!("{}", a_i);
    }
}
