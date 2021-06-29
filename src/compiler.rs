//! 抽象構文木を型チェックして実行可能な式にする．

use crate::{error, pos, program, syntax};
use std::collections::HashMap;

use std::cell::Cell;
use std::rc::Rc;

// Rc は，関数の中身をメモリ上に置いておくのに用いる．

// グローバル変数は global: HashMap<String, Value> として保管
// Value には RcCell<f64>, RcCell<bool> などが入る．関数も入る
// compile 時にこれらは Reference の形で式の中に入る

// syntax::Statement::Expression
// compile するときに global を参照，実行．
// 関数呼び出し
//
//

// 引数 Vec<Value>
// オプション引数 HashMap<String, Value>
// ローカル変数 Vec<Value>
// そしてこれらは HashMap<String, Value> にも入れておく（コンパイル後は消える）
