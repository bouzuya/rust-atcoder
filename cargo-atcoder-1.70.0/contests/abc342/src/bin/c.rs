use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        q: usize,
        cd: [(char, char); q],
    };
    let s = s
        .iter()
        .copied()
        .map(|c| (c as u8 - b'a') as usize)
        .collect::<Vec<usize>>();
    let cd = cd
        .iter()
        .copied()
        .map(|(c, d)| ((c as u8 - b'a') as usize, (d as u8 - b'a') as usize))
        .collect::<Vec<(usize, usize)>>();
    let mut table = (0..26).collect::<Vec<usize>>();
    for (c, d) in cd {
        for i in 0..26 {
            if table[i] == c {
                table[i] = d;
            }
        }
    }
    for s_i in s {
        let c = table[s_i];
        print!("{}", (c as u8 + b'a') as char);
    }
    println!();
}
