use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };
    let sum_r = a
        .iter()
        .map(|a_i| a_i.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    let sum_c = {
        let mut s = vec![];
        for j in 0..w {
            let mut s_j = 0;
            for i in 0..h {
                s_j += a[i][j];
            }
            s.push(s_j);
        }
        s
    };
    let mut ans = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            ans[i][j] = sum_r[i] + sum_c[j] - a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}{}", ans[i][j], if j == w - 1 { '\n' } else { ' ' });
        }
    }
}
