use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use maplit::{hashmap, hashset};

type Word = [char; 2];

static FST_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/map.fst"));

lazy_static! {
    /// state machine for the translation
    static ref FST: fst::Map<&'static [u8]> = fst::Map::new(FST_DATA).unwrap();

    // thanks https://github.com/bosondata/simplet2s-rs/blob/master/src/lib.rs#L8 for this special logic
    // Traditional Chinese -> Not convert case
    static ref T2S_EXCLUDE: HashMap<char, HashSet<Word>> = {
        hashmap!{
            '兒' => hashset!{['兒','寬']},
            '覆' => hashset!{['答', '覆'], ['批','覆'], ['回','覆']},
            '夥' => hashset!{['甚','夥']},
            '藉' => hashset!{['慰','藉'], ['狼','藉']},
            '瞭' => hashset!{['瞭','望']},
            '麽' => hashset!{['幺','麽']},
            '幺' => hashset!{['幺','麽']},
            '於' => hashset!{['樊','於']}
        }
    };
    // Traditional Chinese -> Special convert cases ( only convert in certain case )
    static ref T2S_SPECIAL_CONVERT_TYPE: HashMap<char, HashMap<Word, char>> = {
        hashmap!{
            // not convert these chars if not in special cases
            '藉' => hashmap!{['藉','口'] => '借', ['憑','藉'] => '借'},
            '著' => hashmap!{['看','著'] => '着'},
            '苧' => hashmap!{['苧','麻'] => '苎'},
            '乾' => hashmap!{['乾','燥'] => '干', ['乾','爹'] => '干', ['餅','乾'] => '干', ['乾','枯'] => '干', ['乾','旱'] => '干'},
            // convert these chars use naive mapping if not in special cases
            '闔' => hashmap!{['闔','家'] => '合'},
            '鍾' => hashmap!{['鍾','書'] => '锺'},
            '讎' => hashmap!{['校','讎'] => '雠', ['讎','定'] => '雠', ['仇','讎'] => '雠'},
            '畫' => hashmap!{['計','畫'] => '划', ['企','畫'] => '划'},
        }
    };
}

#[inline(always)]
pub fn special_convert(prev: char, cur: char, next: char) -> char {
    let w1 = [prev, cur];
    let w2 = [cur, next];
    if let Some(inner_set) = T2S_EXCLUDE.get(&cur) {
        if inner_set.contains(&w1) || inner_set.contains(&w2) {
            return cur;
        }
    } else if let Some(inner_map) = T2S_SPECIAL_CONVERT_TYPE.get(&cur) {
        if let Some(c) = inner_map.get(&w1) {
            return *c;
        }
        if let Some(c) = inner_map.get(&w2) {
            return *c;
        }
    }
    FST.get(c2b(cur)).map(i2c).unwrap_or(cur)
}

#[inline(always)]
fn c2b(c: char) -> [u8; 4] {
    let i = c as u32;
    i.to_be_bytes()
}

#[inline(always)]
fn i2c(i: u64) -> char {
    // SAFETY: the char is from precompiled FST, so it should exists
    unsafe { char::from_u32_unchecked(i as _) }
}
