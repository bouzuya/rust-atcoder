use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut map = vec![0_usize; 26];
    for c in s {
        map[(c as u8 - b'a') as usize] += 1;
    }
    let max = *map.iter().max().unwrap();
    for i in 0..26 {
        if map[i] == max {
            println!("{}", (i as u8 + b'a') as char);
            return;
        }
    }
}
