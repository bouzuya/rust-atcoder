use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    let mut color = vec![vec![' '; b * n]; a * n];
    for i_t in 0..n {
        for j_t in 0..n {
            for i in 0..a {
                for j in 0..b {
                    color[i_t * a + i][j_t * b + j] = match (i_t + j_t) % 2 {
                        0 => '.',
                        1 => '#',
                        _ => unreachable!(),
                    }
                }
            }
        }
    }
    for i in 0..a * n {
        for j in 0..b * n {
            print!("{}", color[i][j]);
        }
        println!();
    }
}
