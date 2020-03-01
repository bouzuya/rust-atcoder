use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        scv: [(usize, usize); m]
    };

    for i in 0..10usize.pow(n as u32) {
        let s = format!("{}", i);
        if s.len() != n {
            continue;
        }
        let cv: Vec<char> = s.chars().collect();
        if scv
            .iter()
            .all(|&(si, ci)| cv[si - 1] as u8 == ci as u8 + b'0')
        {
            println!("{}", s);
            return;
        }
    }
    println!("{}", -1);
}
