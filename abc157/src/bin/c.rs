use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        scv: [(usize, char); m]
    };

    for i in 0..10usize.pow(n as u32) {
        let s = format!("{}", i);
        if s.len() != n {
            continue;
        }
        let cv = s.chars().collect::<Vec<char>>();
        if scv.iter().all(|&(si, ci)| cv[si - 1] == ci) {
            println!("{}", s);
            return;
        }
    }
    println!("{}", -1);
}
