use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h]
    };
    for i in 0..h {
        for j in 0..w {
            print!(
                "{}",
                if a[i][j] == 0 {
                    '.'
                } else {
                    ((a[i][j] - 1) as u8 + b'A') as char
                }
            );
        }
        println!();
    }
}
