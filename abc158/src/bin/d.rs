use proconio::input;

fn main() {
    input! {
        s: String,
        q: usize,
    }
    let mut r = false;
    let mut v = vec![vec![]; 2];
    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => r = !r,
            2 => {
                input! { f: usize, c: char }
                match f {
                    1 => v[if !r { 0 } else { 1 }].push(c),
                    2 => v[if !r { 1 } else { 0 }].push(c),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    let f: String = v[if !r { 0 } else { 1 }].iter().rev().collect();
    let c: String = if !r { s } else { s.chars().rev().collect() };
    let b: String = v[if !r { 1 } else { 0 }].iter().collect();
    println!("{}{}{}", f, c, b);
}
