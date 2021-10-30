use std::slice::from_raw_parts_mut;

use fst::Map;
use lazy_static::lazy_static;

static FST_DATA: &[u8] = include_bytes!("../map.fst");

lazy_static! {
    /// state machine for the translation
    static ref FST: Map<&'static [u8]> = Map::new(FST_DATA).unwrap();
}

/// convert the string containing any tranditional Chinese to simplified Chinese
pub fn convert(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.len_utf8() == 3 {
                FST.get(c2b(c)).map(i2c).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

/// modify the string containing any tranditional Chinese to simplified Chinese
pub fn replace(s: &mut String) {
    let ptr = s.as_mut_ptr();
    let len = s.len();
    let s1 = unsafe { from_raw_parts_mut(ptr, len) };
    for (pos, c) in s.char_indices() {
        if c.len_utf8() == 3 {
            let c = FST.get(c2b(c)).map(i2c).unwrap_or(c);
            c.encode_utf8(&mut s1[pos..pos + 3]);
        }
    }
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    lazy_static! {
        static ref TESTS: HashMap<&'static str, &'static str> = {
            let mut map = HashMap::new();
            map.insert(
                "《第一批异体字整理表》已將「託」與「托」合併為「托」",
                "《第一批异体字整理表》已将「托」与「托」合并为「托」",
            );
            map.insert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。", "「于」曾被《第一批异体字整理表》视为「于」的异体字废除，后来恢复为规范字，但只用作姓氏人名，如樊于期，其他情况仍用「于」。");
            map.insert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。", "「藉」其他意义仍然保留的，藉口、凭藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。");
            map.insert(
                "企畫 計畫 企劃 計劃 畫圖 畫畫",
                "企画 计画 企划 计划 画图 画画",
            );
            map.insert(
                "英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成",
                "英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成",
            );
            map
        };
    }

    #[test]
    fn t2s_convert_works() {
        for (k, v) in TESTS.iter() {
            let s = convert(k);
            assert_eq!(&s, v);
        }
    }

    #[test]
    fn t2s_replace_works() {
        for (k, v) in TESTS.iter() {
            let mut s = k.to_string();
            replace(&mut s);
            assert_eq!(&s, v);
        }
    }
}
