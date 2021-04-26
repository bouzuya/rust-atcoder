use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut s = vec![];
    for bits in 0..1 << n {
        let ps = (0..n)
            .map(|i| if (bits >> i) & 1 == 1 { 1 } else { -1 })
            .collect::<Vec<i64>>();
        let mut ok = true;
        let mut sum = 0;
        for &p_i in ps.iter() {
            sum += p_i;
            if sum < 0 {
                ok = false;
            }
        }
        if sum != 0 {
            ok = false;
        }
        if ok {
            let t = ps
                .iter()
                .map(|&p_i| if p_i == 1 { '(' } else { ')' })
                .collect::<String>();
            s.push(t);
        }
    }
    s.sort();
    for s_i in s.iter() {
        println!("{}", s_i);
    }
}
