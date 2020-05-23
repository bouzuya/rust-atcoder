use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut c = vec![0; n];
    for &a_i in a.iter() {
        c[a_i] += 1;
    }
    let mut ans = None;
    for (i, &c_i) in c.iter().enumerate() {
        match c_i {
            0 => {
                let x = i + 1;
                ans = Some(match ans {
                    None => (x, 0_usize),
                    Some((_, y)) => (x, y),
                });
            }
            1 => {}
            2 => {
                let y = i + 1;
                ans = Some(match ans {
                    None => (0_usize, y),
                    Some((x, _)) => (x, y),
                });
            }
            _ => unreachable!(),
        }
    }
    match ans {
        None => println!("Correct"),
        Some((x, y)) => println!("{} {}", y, x),
    }
}
