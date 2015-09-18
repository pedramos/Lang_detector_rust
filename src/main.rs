use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;
use std::io;



fn read_file(data: &str) -> HashMap<String,u32> {

    println!("Reading file {}",data);

    let  file = File::open(data);
    assert!(file.is_ok(), "Couldn't open file");

    let buf = BufReader::new(file.unwrap());
    let mut hash = HashMap::new();
    let mut split_1 = String::new();
    let mut split_2 = String::new();

    for line in buf.lines() {
        assert!(line.is_ok(), "falou a ler linha");
        let linha = line.unwrap();
        //println!("linha: {}",linha);
        let splited = linha.split_whitespace().collect::<Vec<_>>();
        //println!("tri from vec: {}",splited[0]);
        //println!("value from vec: {}",splited[1]);
        if splited.len() < 3 {
            split_1.push_str(splited[0]);
            split_2.push_str(splited[1]);
        }
        else {
            split_1.push_str(splited[0]);
            split_1.push(' ');
            split_1.push_str(splited[1]);
            split_2.push_str(splited[2]);

        }
        //println!("{}", split_1);
        //println!("{}", split_2);
        hash.insert(split_1.to_string(),split_2.parse::<u32>().unwrap());
        split_1.clear();
        split_2.clear();

    }
    hash
}
//Function to separete the sentence in groups of 3 letters
//FALTA TESTAR
fn create_trip (input: &str) -> Vec<String> {

    let input_vec  = input.chars().collect::<Vec<char>>().clone();
    let x = input.len();
    if x<3 {
        println!("Please insert a bigger sentence");
        let result = Vec::with_capacity(0);
        return result;
    }
    let capacity = x - 2;
    let mut result = Vec::with_capacity(capacity);

    for i in (0 .. capacity){
        result[i]=String::new();
        result[i].push(input_vec[i]);
        result[i].push(input_vec[i+1]);
        result[i].push(input_vec[i+2]);

    }
    result
}




fn main() {
    let mut program_input =String::new();
    let pt= read_file("/home/rami/cool_stuff/Lang_detector_rust/src/pt_trigram_count_pruned_100000.tsv");
    loop {
        print!("Insert here the phrase:\n>>");
        io::stdin().read_line(&mut program_input);
        

    }
}
