use proconio::input;

fn dfs(count: &mut usize, buf: &mut Vec<usize>, n: usize) {
    if !buf.is_empty() {
        let x = buf
            .repeat(2)
            .into_iter()
            .map(|i| (i as u8 + b'0') as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        if x <= n {
            *count += 1;
        } else {
            return;
        }
        for d in 0..=9 {
            buf.push(d);
            dfs(count, buf, n);
            buf.pop();
        }
    } else {
        for d in 1..=9 {
            buf.push(d);
            dfs(count, buf, n);
            buf.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0_usize;
    let mut buf = vec![];
    dfs(&mut count, &mut buf, n);
    let ans = count;
    println!("{}", ans);
}
