use proconio::input;

fn main() {
    input! {
        n: usize,
        pv: [(char, String); n]
    };
    let mut ids = vec![None; 6];
    for (i, (p, v)) in pv.into_iter().enumerate() {
        if v != "AC" {
            continue;
        }
        let p = (p as u8 - b'A') as usize;
        match ids[p] {
            Some(_) => {}
            None => ids[p] = Some(i),
        }
    }
    for id in ids {
        println!("{}", id.unwrap() + 1);
    }
}
