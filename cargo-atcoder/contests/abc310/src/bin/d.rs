use proconio::{input, marker::Usize1};

fn dfs(
    n: usize,
    t: usize,
    ab: &[(usize, usize)],
    count: &mut usize,
    team_num: usize,
    team: &mut Vec<usize>,
    i: usize,
) {
    if i == n {
        if team_num != t {
            return;
        }
        let mut ok = true;
        for (a, b) in ab.iter().copied() {
            if team[a] == team[b] {
                ok = false;
                break;
            }
        }
        if ok {
            *count += 1;
        }
        return;
    }

    for j in 0..team_num + 1 {
        team.push(j);
        dfs(
            n,
            t,
            ab,
            count,
            team_num + if j == team_num { 1 } else { 0 },
            team,
            i + 1,
        );
        team.pop();
    }
}

fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut team = vec![0];
    let mut count = 0_usize;
    dfs(n, t, &ab, &mut count, 1, &mut team, 1);
    let ans = count;
    println!("{}", ans);
}
