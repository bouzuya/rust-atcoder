use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 1 << n],
    };
    let mut xs = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (i, a_i))
        .collect::<Vec<(usize, usize)>>();
    if xs.len() == 2 {
        let ans = if xs[0].1 > xs[1].1 { xs[1].0 } else { xs[0].0 };
        println!("{}", ans + 1);
        return;
    }
    loop {
        let mut ys = vec![];

        for i in 0..xs.len() {
            if i % 2 != 0 {
                continue;
            }
            if xs[i].1 > xs[i + 1].1 {
                ys.push(xs[i]);
            } else {
                ys.push(xs[i + 1]);
            }
        }

        if ys.len() == 2 {
            let ans = if ys[0].1 > ys[1].1 { ys[1].0 } else { ys[0].0 };
            println!("{}", ans + 1);
            return;
        }
        xs = ys;
    }
}
