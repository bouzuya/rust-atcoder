use proconio::input;

fn dfs(s: &mut Vec<usize>, t: &Vec<Vec<u8>>, n: usize, k: usize) -> bool {
    if s.len() == n {
        let mut v = 0;
        for (i, &j) in s.iter().enumerate() {
            v ^= t[i][j];
        }
        return v == 0;
    }

    for j in 0..k {
        s.push(j);
        if dfs(s, t, n, k) {
            return true;
        }
        s.pop();
    }
    false
}

fn main() {
    input! {
        n: usize,
        k: usize,
        t: [[u8; k]; n],
    };
    let found = dfs(&mut vec![], &t, n, k);
    let ans = if found { "Found" } else { "Nothing" };
    println!("{}", ans);
}
