use special::special_convert;
use std::slice::from_raw_parts_mut;

mod macros;
mod special;

/// convert the string containing any tranditional Chinese to simplified Chinese
pub fn convert(s: &str) -> String {
    if s.is_empty() {
        return "".to_string();
    }

    let mut ret = String::with_capacity(s.len());

    let mut chars = s.chars();
    let mut prev = '0';
    // String shall have at least one char
    let mut cur = chars.next().unwrap();
    let mut next;

    for c in chars {
        next = c;
        if cur.len_utf8() == 3 {
            let converted = special_convert(prev, cur, next);
            ret.push(converted);
        } else {
            ret.push(cur);
        }
        prev = cur;
        cur = c;
    }
    next = '0';

    // process last char
    if cur.len_utf8() == 3 {
        let converted = special_convert(prev, cur, next);
        ret.push(converted);
    } else {
        ret.push(cur);
    }

    ret
}

/// modify the string containing any tranditional Chinese to simplified Chinese
pub fn replace(s: &mut String) {
    if s.is_empty() {
        return;
    }

    let ptr = s.as_mut_ptr();
    let len = s.len();
    let s1 = unsafe { from_raw_parts_mut(ptr, len) };
    let mut char_indices = s.char_indices();

    let mut prev = '0';
    // String shall have at least one char
    let (mut cur_pos, mut cur) = char_indices.next().unwrap();
    let mut next;

    for (pos, c) in s.char_indices() {
        next = c;
        if cur.len_utf8() == 3 {
            let converted = special_convert(prev, cur, next);
            converted.encode_utf8(&mut s1[cur_pos..cur_pos + 3]);
        }
        prev = cur;
        cur = c;
        cur_pos = pos;
    }
    next = '0';
    // process last char
    if cur.len_utf8() == 3 {
        let converted = special_convert(prev, cur, next);
        converted.encode_utf8(&mut s1[cur_pos..cur_pos + 3]);
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    use super::*;

    lazy_static! {
        static ref TESTS: HashMap<&'static str, &'static str> = {
            let mut map = HashMap::new();
            map.insert(
                "《第一批异体字整理表》已將「託」與「托」合併為「托」",
                "《第一批异体字整理表》已将「托」与「托」合并为「托」",
            );
            map.insert("「於」曾被《第一批異體字整理表》視為「于」的異體字廢除，後來恢復為規範字，但只用作姓氏人名，如樊於期，其他情況仍用「于」。", "「于」曾被《第一批异体字整理表》视为「于」的异体字废除，后来恢复为规范字，但只用作姓氏人名，如樊於期，其他情况仍用「于」。");
            map.insert("「藉」其他意义仍然保留的，藉口、憑藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。", "「藉」其他意义仍然保留的，藉口、凭藉的藉（jiè）简化作借，慰藉（jiè）、狼藉（jí）等的藉仍用藉。");
            map.insert(
                "企畫 計畫 企劃 計劃 畫圖 畫畫",
                "企划 计划 企划 计划 画图 画画",
            );
            map.insert(
                "英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成",
                "英特尔宣布“漏洞门”应对计划：为5年内90%处理器提供补丁，下周末前完成",
            );
            map.insert(
                "hello world！this is a 非常特殊的企畫。",
                "hello world！this is a 非常特殊的企划。",
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
