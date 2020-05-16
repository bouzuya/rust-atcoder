use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        a: [usize; n],
    };
    let mut x = 0;
    let mut y = 0;
    let mut ans = vec![vec![0_usize; w]; h];
    for (i, &a_i) in a.iter().enumerate() {
        for _ in 0..a_i {
            ans[y][x] = i + 1;
            if y % 2 == 0 {
                if x == w - 1 {
                    y += 1;
                } else {
                    x += 1;
                }
            } else {
                if x == 0 {
                    y += 1;
                } else {
                    x -= 1;
                }
            }
        }
    }
    for a_i in ans.iter() {
        for (x, &a_ij) in a_i.iter().enumerate() {
            print!("{}{}", a_ij, if x == w - 1 { "" } else { " " });
        }
        println!();
    }
}
