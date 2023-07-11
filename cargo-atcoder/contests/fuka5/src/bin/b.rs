use proconio::{input, marker::Chars};

fn main() {
    loop {
        input! {
            d: String,
        }
        if &d == "0" {
            break;
        }

        input! {
            t: String,
            time: Chars,
        }
        let (year, month, day, hour, min, sec) = {
            let y_m_d = d.split('/').collect::<Vec<&str>>();
            let y = y_m_d[0].parse::<usize>().unwrap();
            let m = y_m_d[1].parse::<usize>().unwrap();
            let d = y_m_d[2].parse::<usize>().unwrap();
            let h_m_s = t.split(':').collect::<Vec<&str>>();
            let h = h_m_s[0].parse::<usize>().unwrap();
            let min = h_m_s[1].parse::<usize>().unwrap();
            let s = h_m_s[2].parse::<usize>().unwrap();
            (y, m, d, h, min, s)
        };
        let mut t = 2_usize.pow(time.len() as u32) - 1;

        let x = t % 60 + sec;
        let ans_sec = x % 60;
        t /= 60;
        let x = t % 60 + min + x / 60;
        let ans_min = x % 60;
        t /= 60;
        let x = t % 24 + hour + x / 60;
        let ans_hour = x % 24;
        t /= 24;
        t += x / 24;

        let is_leap_year =
            |y: usize| -> bool { (y % 400 == 0) || ((y % 4 == 0) && (y % 100 != 0)) };
        let calendar = |y: usize| -> Vec<usize> {
            let dm = vec![
                31,
                if is_leap_year(y) { 29 } else { 28 },
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            std::iter::once(0)
                .chain(dm.iter().scan(0, |acc, &i| {
                    *acc += i;
                    Some(*acc)
                }))
                .collect::<Vec<usize>>()
        };

        let cal = calendar(year);
        t += cal[month - 1] + day;

        let mut ans_year = year;
        loop {
            let days = if is_leap_year(ans_year) { 366 } else { 365 };
            if t > days {
                t -= days;
                ans_year += 1;
                continue;
            }
            break;
        }

        let cal = calendar(ans_year);
        let mut m = 0;
        for i in 1..=12 {
            if t <= cal[i] {
                t -= cal[i - 1];
                m = i;
                break;
            }
        }
        let d = t;

        println!(
            "{:04}/{:02}/{:02} {:02}:{:02}:{:02}",
            ans_year, m, d, ans_hour, ans_min, ans_sec
        );
    }
}
