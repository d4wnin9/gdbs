gdbのpeda, gef, pwndbgを切り替えるツール

`.gdbinit`を書き換えているだけ

`echo "source /path/to/.py" >> ~/.gdbs`で初期設定、gdbの起動まで持っていくつもりだったけど思ったよりrustが厳しいので休憩release

ex.
```sh
gdbs peda
```