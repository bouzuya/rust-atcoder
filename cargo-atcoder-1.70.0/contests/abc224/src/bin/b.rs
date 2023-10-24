use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    for i_1 in 0..h {
        for i_2 in i_1 + 1..h {
            for j_1 in 0..w {
                for j_2 in j_1 + 1..w {
                    if a[i_1][j_1] + a[i_2][j_2] > a[i_2][j_1] + a[i_1][j_2] {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
}
