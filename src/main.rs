use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;



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
        split_1.push_str(splited[0]);
        split_2.push_str(splited[1]);
        //println!("{}",split_1);
        //println!("{}",split_2);
        hash.insert(split_1.to_string(),split_2.parse::<u32>().unwrap());
        split_1.clear();
        split_2.clear();

    }
    hash
}
//Function to separete the sentence in groups of 3 letters
fn create_trip (&str input) -> <Vec<String>> {

let X = input.len();
if (X<3){
    println!("Please insert a bigger sentence");
    let mut result = Vec::with_capacity(0);
    return result;
}
let capacity = X - 2;
let mut result = Vec::with_capacity(capacity);

//IMCOMPLETE: ja está criado vector com o tamanho certo, falta separar as coisas

}




fn main() {
    let exemplo= read_file("/home/rami/cool_stuff/lang_detector_rust/src/test.txt");
}
