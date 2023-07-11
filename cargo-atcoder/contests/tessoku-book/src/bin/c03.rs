use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        x: i64,
        a: [i64; d - 1],
        q: usize,
        st: [(Usize1, Usize1); q],
    };

    let mut p = vec![x];
    for a_i in a {
        let p_i = p.last().unwrap() + a_i;
        p.push(p_i);
    }

    for (s, t) in st {
        println!(
            "{}",
            match p[s].cmp(&p[t]) {
                std::cmp::Ordering::Less => (t + 1).to_string(),
                std::cmp::Ordering::Equal => "Same".to_owned(),
                std::cmp::Ordering::Greater => (s + 1).to_string(),
            }
        );
    }
}
