use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };
    for _ in 0..w + 2 {
        print!("#");
    }
    println!();
    for i in 0..h {
        print!("#");
        for j in 0..w {
            print!("{}", a[i][j]);
        }
        println!("#");
    }

    for _ in 0..w + 2 {
        print!("#");
    }
    println!();
}
