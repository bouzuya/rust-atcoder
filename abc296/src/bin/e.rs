use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut ord = vec![None; n];
    let mut index = 1_usize;

    let mut ans = 0_usize;
    for i in 0..n {
        if ord[i].is_some() {
            continue;
        }
        let offset = index;
        let mut v = i;
        loop {
            if let Some(l) = ord[v] {
                if l >= offset {
                    ans += index - l;
                }
                break;
            }
            ord[v] = Some(index);
            v = a[v];
            index += 1;
        }
    }

    println!("{}", ans);
}
