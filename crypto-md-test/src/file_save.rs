extern crate core;

use std::collections::HashMap;
use std::thread;
use std::path::Path;
use std::fs::{File, create_dir_all, OpenOptions};
use std::io::{Write, Read};
use chrono::prelude::Local;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use chrono::Duration;


pub struct WriteData {
    is_runing: Arc<Mutex<bool>> ,
    channel_sender: mpsc::Sender<(String, Vec<i8>)>,
    channel_receiver: Arc<Mutex<mpsc::Receiver<(String, Vec<i8>)>>>,
    files: Arc<Mutex<HashMap<String, File>>>
}

impl WriteData {
    pub fn new() -> WriteData {
        let (channel_sender, channel_receiver) =
            mpsc::channel::<(String, Vec<i8>)>();
        WriteData {
            is_runing: Arc::new(Mutex::new(true)),
            channel_sender,
            channel_receiver: Arc::new(Mutex::new(channel_receiver)),
            files: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn add_order_book(&mut self, venue: String, symbol: String, array: Vec<i8>) {
        let key = format!("{}_{}", venue, symbol);
        self.channel_sender.send((key, array))
            .expect("文件数据信道出现问题")
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        let is_runing = self.is_runing.clone();
        let channel_receiver = self.channel_receiver.clone();
        let files = self.files.clone();
        thread::spawn(move || {
            loop {
                println!("hello");
                {
                    let is_runing = is_runing.lock();
                    match is_runing {
                        Ok(_is_runing) => if !*_is_runing {
                            break
                        },
                        Err(_) => {
                            break
                        }
                    }
                }
                let local_time = Local::now();
                let today = local_time.format("%Y%m%d").to_string();

                let mut venue_symbol: String = "".to_string();
                let mut data: Vec<i8> = vec![];

                {
                    let channel_receiver_lock
                        = channel_receiver.lock();
                    let mut flag = false;
                    if let Ok(channel_receiver_look)
                        = channel_receiver_lock {
                        if let Ok(receiver)
                            = channel_receiver_look.recv() {
                            venue_symbol =receiver.0;
                            data = receiver.1;
                            flag = true;
                        }
                    }
                    if !flag {
                        break;
                    }
                }

                let mut files_lock
                    = if let Ok(files_lock) = files.lock() {
                        files_lock
                    } else {
                        continue;
                    };

                match files_lock.get(&format!("{}_{}", venue_symbol, local_time)) {
                    None => {
                        let take: Vec<&str> = venue_symbol.split("_").collect();
                        let venue = take.get(0).unwrap();
                        let symbol = take.get(1).unwrap();

                        let filename_orderbook = check_path(venue.to_string(), symbol.to_string());
                        let data_file = OpenOptions::new().append(true).open(filename_orderbook).expect("文件无法打开");

                        write_file(&data_file, data);
                        files_lock.insert(today, data_file);

                        let yesterday
                            = (local_time - Duration::seconds(86400)).format("%Y%m%d");
                        let key = &format!("{}_{}", venue_symbol, yesterday);
                        if let Some(_file) = files_lock.get(key) {
                            files_lock.remove(key);
                        }

                    }
                    Some(file) => {
                        write_file(file, data);
                    }
                };
            }
        })
    }

}


pub struct ReadData{
    file: File
}

impl ReadData {
    pub fn new(venue: String, symbol: String, day: i64) -> Option<ReadData> {

    let local_time = Local::now();
    let day_time = local_time - Duration::seconds(86400*day);
    let day_format = day_time.format("%Y%m%d");


    let path = &format!("./record/{}/", day_format);


    let filename_orderbook = if venue.len() > 5 && &venue[venue.len()-5..] == "_SWAP" {
        format!("{}{}_{}_book_log.csv", path, venue, symbol)
    } else {
        format!("{}{}_{}SWAP_book_log.csv", path, venue, symbol)
    };

    println!("{}", filename_orderbook);
    if !Path::new(&filename_orderbook).is_file() {
        // 文件不存在
        return None;
    }

    let file = File::open(filename_orderbook).unwrap();

    Some(ReadData {
        file
    })

    }
}

// http utp utp:quic 
impl Iterator for ReadData {
    type Item = Vec<i8>;

    // 可优化
    fn next(&mut self) -> Option<Self::Item> {
        let mut data_len = [0u8;2];
        if let Ok(_data_size) = self.file.read(&mut data_len) {
            if 2 != _data_size {
                return None;
            }
            let mut data: Vec<i8> = Vec::new();
            let data_len = u16::from_be_bytes(data_len)  as usize;
            let mut byte = [0u8;1];
            for _i in 0..data_len {
                if let Result::Err(_error_msg) = self.file.read(&mut byte){
                    return None;
                }
                data.push(byte[0] as i8);
            }
            return Some(data);
        }
        None
    }
}


// 可优化
pub fn write_file (mut file: &File, data: Vec<i8>) {
    let data_len = &(data.len() as i16).to_be_bytes();
    file.write_all(data_len).expect("长度计算失败");
    for i in data {
        file.write_all(&[i as u8]).expect("");
    }
}

pub fn check_path(venue: String, symbol: String) -> String {
    let local_time = Local::now().format("%Y%m%d").to_string();
    let path = &format!("./record/{}/", local_time);
    create_dir_all(path).expect("目录创建失败");

    let filename_orderbook = if venue.len() > 5 && &venue[venue.len()-5..] == "_SWAP" {
        format!("{}{}_{}_book_log.csv", path, venue, symbol)
    } else {
        format!("{}{}_{}SWAP_book_log.csv", path, venue, symbol)
    };

    println!("{}", filename_orderbook);
    if !Path::new(&filename_orderbook).is_file() {
        File::create(&filename_orderbook).expect("创建文件失败");
    }
    filename_orderbook
}




fn main() {

    // 添加数据
    // let mut a = WriteData::new();
    // a.add_order_book("name".to_string(), "ydf".to_string(), vec![104, 101, 108, 108, 111]);
    // a.add_order_book("name".to_string(), "ydf".to_string(), vec![32]);
    // a.add_order_book("name".to_string(), "ydf".to_string(), vec![119, 111, 114, 108, 100]);
    // let c = a.start();
    // c.join().unwrap();


    // 读取文件
    if let Some(read_data) = ReadData::new("name".to_string(), "ydf".to_string(), 0) {
        for data in read_data {
            println!("{:?}", data);
        }
    }

}