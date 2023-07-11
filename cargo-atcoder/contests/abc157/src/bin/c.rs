use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(Usize1, char); m],
    };
    let mut ans = vec![' '; n];
    for (s, c) in sc {
        if ans[s] == ' ' {
            ans[s] = c;
        } else if ans[s] != c {
            println!("-1");
            return;
        }
    }
    if n == 1 {
        if ans[0] == ' ' {
            println!("0");
        } else {
            println!("{}", ans[0]);
        }
        return;
    }
    for x in 10_usize.pow((n - 1) as u32)..=10_usize.pow(n as u32) - 1 {
        let cs = x.to_string().chars().collect::<Vec<char>>();
        let mut ok = true;
        for i in 0..n {
            if ans[i] == ' ' {
                continue;
            }
            if ans[i] == cs[i] {
                continue;
            }
            ok = false;
            break;
        }
        if ok {
            println!("{}", cs.into_iter().collect::<String>());
            return;
        }
    }
    println!("-1");
}
