gdbのpeda, gef, pwndbgを切り替えるツール

```install
 cargo install --git https://github.com/d4wnin9/gdbs.git --branch=main
```

`.gdbinit`を書き換えているだけ

`echo "source /path/to/.py" >> ~/.gdbs`で初期設定、

ex.
```sh
gdbs peda
```


gdbの起動まで持っていくつもりだったけど思ったよりrustが厳しいので休憩release

ファイルのパス指定を"~/"でできなくて修正