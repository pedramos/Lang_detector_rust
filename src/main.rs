use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;


fn read_file(data: &str){

    println!("Reading file {}",data);

    let  file = File::open(data);

    let buf = BufReader::new(file.unwrap());
    let mut hash = HashMap::new();

    let mut split_1 = String::new();
    let mut split_2 = String::new();
    for line in buf.lines() {
        split_1.push_str(line.unwrap().split(" ").nth(0).unwrap());
        split_2.push_str(line.unwrap().split(" ").nth(1).unwrap());
        println!("{}",split_1);
        println!("{}",split_2);
        hash.insert(split_1,split_2.parse::<u32>().unwrap());
        split_1.clean();
        split_2.clean();
    }



}

fn main() {
    read_file("/home/rami/cool_stuff/Lang_detector_rust/src/test.txt")
}
