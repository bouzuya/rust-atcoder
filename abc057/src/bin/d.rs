use combination::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        mut v: [i64; n],
    };

    v.sort_by_key(|&v_i| -v_i);

    let avg = v[0..a].iter().sum::<i64>() as f64 / a as f64;
    let count = {
        let combin = CombinationTable::new(n);
        let mut x = 0;
        let mut y = 0;
        for (i, &v_i) in v.iter().enumerate() {
            if v_i == v[a - 1] {
                x += 1;
                if i < a {
                    y += 1;
                }
            }
        }
        if y == a {
            (a..=b).map(|y| combin.combination(x, y)).sum::<usize>()
        } else {
            combin.combination(x, y)
        }
    };

    println!("{:.6}", avg);
    println!("{}", count);
}

mod combination {
    pub struct CombinationTable {
        // pascal's triangle
        c: Vec<Vec<usize>>,
    }

    impl CombinationTable {
        pub fn new(n: usize) -> Self {
            let mut c = vec![vec![0; n + 1]; n + 1];
            for i in 0..n + 1 {
                for j in 0..i + 1 {
                    if j == 0 || j == i {
                        c[i][j] = 1;
                    } else {
                        c[i][j] = c[i - 1][j - 1] + c[i - 1][j];
                    }
                }
            }
            CombinationTable { c }
        }

        pub fn combination(&self, n: usize, k: usize) -> usize {
            self.c[n][k]
        }
    }
}
