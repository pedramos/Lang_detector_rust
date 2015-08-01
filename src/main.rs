use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;


fn read_file(data: &str){

    println!("Reading file {}","/home/rami/cool_stuff/Lang_detector_rust/src/test.txt");

    let  file = File::open(data);

    let buf = BufReader::new(file.unwrap());
    //let mut hash = HashMap::new();

    let mut split_1 = String::new();
    let mut split_2 = String::new();
    for line in buf.lines() {
        let mut splited = line.unwrap().split(" ");
        //println!("{}", line.unwrap().split(" ").nth(0).unwrap());
        println!("{}", splited.nth(1).unwrap());
        //split_1.push_str(line.unwrap().split(" ").nth(0).unwrap());
        //split_2.push_str(line.unwrap().split(" ").nth(1).unwrap());
        //println!("{}",split_1);
        //println!("{}",split_2);
        //hash.insert(split_1,split_2.parse::<u32>().unwrap());
        //split_1.clear();
        //split_2.clear();


    }
}






fn main() {
    read_file("/home/rami/cool_stuff/Lang_detector_rust/src/test.txt")
}
