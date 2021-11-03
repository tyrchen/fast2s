use std::io::{self, BufRead};
use std::{env, fs::File, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/t2s.txt");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_file = Path::new(&out_dir).join("map.bin");
    let f = File::create(&out_file).unwrap();
    let v = get_kv("src/t2s.txt");
    bincode::serialize_into(f, &v).unwrap();
}

fn s2c(s: &str) -> char {
    let mut chars = s.chars();
    let c = chars.next().unwrap();
    assert!(chars.next().is_none());
    assert!(c.len_utf8() == 3);
    c
}

fn get_kv(filename: &str) -> Vec<(char, char)> {
    let f = File::open(filename).unwrap();
    let lines = io::BufReader::new(f).lines();
    let mut v = Vec::with_capacity(4096);
    for line in lines {
        let line = line.unwrap();
        let kv: Vec<_> = line.split(' ').collect();
        v.push((s2c(kv[0]), s2c(kv[1])));
    }

    v
}
