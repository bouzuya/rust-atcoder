use proconio::input;

fn main() {
    input! {
        s: String,
        q: usize,
    }
    let mut d = 0;
    let mut v = vec![vec![]; 2];
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                d = if d == 0 { 1 } else { 0 };
            }
            2 => {
                input! { f: usize, c: char }
                match f {
                    1 => v[d].push(c),
                    2 => v[if d == 0 { 1 } else { 0 }].push(c),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    let f: String = v[d].iter().rev().collect();
    let c: String = if d % 2 == 0 {
        s.chars().collect()
    } else {
        s.chars().rev().collect()
    };
    let b: String = v[if d == 0 { 1 } else { 0 }].iter().collect();
    print!("{}", f);
    print!("{}", c);
    println!("{}", b);
}
