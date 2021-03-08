use log::error;
use std::process::exit;

// これ使うと楽だけど、エラーの起きた
// ファイル名・行番号が機能しなくなる
// 場合によってはas_str()呼ばないと行けないのでマクロにしてみても良いかも
pub fn print_and_exit(message: &str) -> ! {
    error!("{}", message);
    exit(-1);
}