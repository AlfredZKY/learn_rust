extern crate bit_set;
extern crate rand;
extern crate time;
use bit_set::BitSet;
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use time::*;

// 生成1千万的随机数
pub fn init() {
    const FILE_NAME: &'static str = "sorting/sort.txt";
    let mut file = File::create(FILE_NAME).expect("file create fail");
    file.write_all(b"").expect("file write fail");
    let mut rd = rand::thread_rng();
    for _ in 0..10000000 {
        let tand = rd.gen_range(1, 200000000).to_string() + "\n";
        file.write(tand.as_bytes()).expect("file write fail");
    }
}

pub fn sort_by_self(limit: u32, offset: u32) {
    let mut byte: Vec<u32> = vec![0; 1994627];
    let mut file = File::open("sorting/sort.txt").expect("file create fail");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let hh = buffer.split("\n");
    for ll in hh {
        match ll.parse::<u32>() {
            Ok(num) => {
                set_bit(&mut byte, num);
            }
            Err(_) => println!("That was not a number"),
        }
    }

    let mut my_file = File::create("sorting/new.txt").expect("file create fail");
    for x in 0..20000000 {
        let aa = get_bit(&mut byte, x);
        if aa != 0 {
            my_file
                .write((x.to_string() + "\n").as_bytes())
                .expect("file write fail");
        }
    }
}

fn set_bit(byte: &mut Vec<u32>, n: u32) {
    byte[(n / 32) as usize] |= 1 << (n % 32); // 设置
}

fn get_bit(byte: &mut Vec<u32>, n: u32) -> u32 {
    return (byte[(n / 32) as usize] & (1 << (n % 32)));
}
