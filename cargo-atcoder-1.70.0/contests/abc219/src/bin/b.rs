use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [String; 3],
        t: Chars,
    }

    for t_i in t {
        let index = (t_i as u8 - b'1') as usize;
        print!("{}", s[index]);
    }
    println!();
}
