use proconio::input;

fn dfs(n: usize, l: usize, res: &mut Vec<char>) -> usize {
    if l == res.len() {
        return if "357".chars().all(|c| res.contains(&c))
            && res.iter().collect::<String>().parse::<usize>().unwrap() <= n
        {
            1
        } else {
            0
        };
    }
    let mut count = 0;
    for c in "357".chars() {
        res.push(c);
        count += dfs(n, l, res);
        res.pop();
    }
    count
}

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0;
    for l in 3..=10 {
        count += dfs(n, l, &mut vec![]);
    }
    let ans = count;
    println!("{}", ans);
}
