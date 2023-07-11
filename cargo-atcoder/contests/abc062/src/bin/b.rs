use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };
    println!("{}", "#".repeat(w + 2));
    for i in 0..h {
        print!("#");
        for j in 0..w {
            print!("{}", a[i][j]);
        }
        println!("#");
    }
    println!("{}", "#".repeat(w + 2));
}
