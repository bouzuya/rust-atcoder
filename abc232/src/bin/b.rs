use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let s = s
        .into_iter()
        .map(|s_i| (s_i as u8 - b'a') as usize)
        .collect::<Vec<usize>>();
    let t = t
        .into_iter()
        .map(|t_i| (t_i as u8 - b'a') as usize)
        .collect::<Vec<usize>>();
    for k in 1..=26 {
        let mut ok = true;
        for (s_i, t_i) in s.iter().copied().zip(t.iter().copied()) {
            if (s_i + k) % 26 != t_i {
                ok = false;
                break;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
