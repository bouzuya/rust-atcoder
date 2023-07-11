use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; n],
    };
    let mut is = vec![0; n];
    for (i, p_i) in p.iter().enumerate() {
        is[p_i - 1] = i;
    }

    let mut ans = vec![];
    let mut i = 0;
    while i < n - 1 {
        let j = is[i];
        if j <= i {
            println!("-1");
            return;
        }
        for k in (i..j).rev() {
            p.swap(k, k + 1);
            ans.push(format!("{}", k + 1));
            if ans.len() > n - 1 {
                println!("-1");
                return;
            }
        }
        i = j;
    }
    if ans.len() != n - 1 {
        println!("-1");
        return;
    }
    if !p.iter().enumerate().all(|(i, p_i)| p_i == &(i + 1)) {
        println!("-1");
        return;
    }
    println!("{}", ans.join("\n"));
}
