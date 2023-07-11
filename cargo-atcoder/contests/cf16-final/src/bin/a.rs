use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [[String; w]; h],
    };
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == "snuke" {
                let a = (c as u8 + 'A' as u8) as char;
                let n = r + 1;
                println!("{}{}", a, n);
            }
        }
    }
}
