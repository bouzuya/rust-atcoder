use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut a: [usize; n],
    };
    let mut index = 0_usize;
    let mut ans = vec![vec![0_usize; w]; h];
    for i in 0..h {
        let range = if i % 2 == 0 {
            (0..w).collect::<Vec<usize>>()
        } else {
            (0..w).rev().collect::<Vec<usize>>()
        };
        for j in range {
            if a[index] == 0 {
                index += 1;
            }
            ans[i][j] = index + 1;
            a[index] -= 1;
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}{}", ans[i][j], if j == w - 1 { '\n' } else { ' ' });
        }
    }
}
