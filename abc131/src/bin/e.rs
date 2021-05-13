use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    if 2 * k > (n - 1) * (n - 2) {
        println!("-1");
        return;
    }
    let mut es = vec![];
    for i in 1..n {
        es.push((0, i));
    }
    let mut count = 0;
    'for_u: for u in 1..n {
        for v in u + 1..n {
            if (n - 1) * (n - 2) - 2 * count == 2 * k {
                break 'for_u;
            }
            es.push((u, v));
            count += 1;
        }
    }
    println!("{}", es.len());
    for (u, v) in es {
        println!("{} {}", u + 1, v + 1);
    }
}
