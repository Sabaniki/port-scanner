# port-scanner

## これは何
Rustの勉強とネットワークの気持ちの理解のために作ったポートスキャンツール。[Rustで始めるネットワークプログラミング ](https://www.amazon.co.jp/gp/product/B07SW2GXVF/ref=ppx_yo_dt_b_d_asin_title_o02?ie=UTF8&psc=1) を参考に、勉強のために色々と手を加えてます。

## 使い方
カレントディレクトリ内に、以下のような情報を書き込んだyamlファイルをenv.ymlとして作成してください。
```yaml
# [env.yml]
my_ip_address: "127.0.0.1"
my_port: 33333
maximum_port_num: 1023
```
実行時は
`sudo port-scanner <target for port scan> <syn|fin|xmas|null>`として、ターゲットのIPv4アドレスを指定し、スキャン方法を4種類の中から選択してください。
`port-scanner <--help|-h>`でヘルプが見られます。