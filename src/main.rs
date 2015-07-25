use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;


fn read_file(data: &str){

    println!("Reading file {}",data);

    let  file = File::open("/home/rami/cool_stuff/Lang_detector_rust/src/test.txt");

    let buf = BufReader::new(file.unwrap());
    for line in buf.lines() {
        println!("{}", line.unwrap());
    }


}

fn main() {
    read_file("test.txt")
}
