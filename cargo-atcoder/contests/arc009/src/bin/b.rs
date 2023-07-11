use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        b: [usize; 10],
        n: usize,
        a: [Chars; n],
    };
    let mut d = b
        .iter()
        .enumerate()
        .map(|(i, &b_i)| (b_i, i))
        .collect::<Vec<(usize, usize)>>();
    d.sort();
    let mut c = vec![];
    for a_i in a.iter() {
        let mut c_i = 0_usize;
        for &a_ij in a_i.iter() {
            c_i *= 10;
            c_i += d[(a_ij as u8 - '0' as u8) as usize].1;
        }
        c.push((c_i, a_i));
    }
    c.sort();
    for &(_, a_i) in c.iter() {
        for a_ij in a_i {
            print!("{}", a_ij);
        }
        println!();
    }
}
