# AtCoder Problems Virtual Contests

```console
$ # oj-api のインストール
$ # <https://pypi.org/project/online-judge-api-client/>
$ pip3 install online-judge-api-client
# ...

$ # cargo-compete のインストール
$ # <https://github.com/qryxip/cargo-compete>
$ cargo install cargo-compete
# ...

$ cargo compete --version
cargo-compete 0.9.0

$ # 初回のみ
$ mkdir atcoder-problems-virtual-contests
$ cd atcoder-problems-virtual-contests/
$ cargo compete init atcoder
Do you use crates on AtCoder?
1 No
2 Yes
3 Yes, but I submit base64-encoded programs
1..3: 2
# ...

$ cargo compete login atcoder

$ vi compete.toml # update [new]
# [new]
# kind = "oj-api"
# url = "https://kenkoooo.com/atcoder/#/contest/show/{{ id }}"
# path = "./{{ contest }}"

$ # コンテストごと
$ # <https://kenkoooo.com/atcoder/#/contest/show/f428868b-527f-4ee2-bec7-810686bcc24d> の場合
$ cargo compete new 'f428868b-527f-4ee2-bec7-810686bcc24d'
# ...

$ code .
$ cd f428868b-527f-4ee2-bec7-810686bcc24d/

$ # 問題ごと
$ # edit src/bin/0.rs
$ cargo compete submit 0
$ # cargo compete test 0
```
