use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut count = vec![0_usize; n];
    for start in 0..n {
        if count[start] > 0 {
            continue;
        }

        let mut cur = start;
        while count[cur] == 0 {
            count[cur] += 1;
            cur = a[cur];
        }
        let mut ans = vec![];
        while count[cur] == 1 {
            ans.push(cur);
            count[cur] += 1;
            cur = a[cur];
        }

        println!("{}", ans.len());
        let mut s = String::new();
        for (i, a_i) in ans.iter().copied().enumerate() {
            s.push_str(&format!(
                "{}{}",
                (a_i + 1).to_string(),
                if i == ans.len() - 1 { '\n' } else { ' ' }
            ));
        }
        print!("{}", s);
        break;
    }
}
