use fst::MapBuilder;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=t2s.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let data = get_sorted_kv("t2s.txt");
    let out_file = Path::new(&out_dir).join("map.fst");

    build_map(data, out_file);
}

fn build_map(data: Vec<([u8; 4], u64)>, out_file: impl AsRef<Path>) {
    let wtr = io::BufWriter::new(File::create(out_file).unwrap());
    let mut build = MapBuilder::new(wtr).unwrap();
    for (k, v) in data.into_iter() {
        build.insert(k, v).unwrap();
    }
    build.finish().unwrap();
}

fn s2c(s: &str) -> char {
    let mut chars = s.chars();
    let c = chars.next().unwrap();
    assert!(chars.next().is_none());
    assert!(c.len_utf8() == 3);
    c
}

fn convert_to_bytes(s: &str) -> [u8; 4] {
    let i = s2c(s) as u32;
    i.to_be_bytes()
}

fn convert_to_u64(s: &str) -> u64 {
    s2c(s) as u64
}

fn get_sorted_kv(filename: &str) -> Vec<([u8; 4], u64)> {
    let f = File::open(filename).unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut v = Vec::with_capacity(4096);
    for line in lines {
        let line = line.unwrap();
        let kv: Vec<_> = line.split(' ').collect();
        v.push((convert_to_bytes(kv[0]), convert_to_u64(kv[1])));
    }

    // fst want sorted data, so we need to sort the vector to correct order
    v.sort_by(|(k1, _), (k2, _)| k1.partial_cmp(k2).unwrap());

    v
}
