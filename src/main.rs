fn read_file(data: &std){

    println!("Reading file {}",data);


    //CENAS sacadas da documentação. Falta adaptar
    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));

    //CEnas melhores para ler ficheiros
    use std::io::{BufReader,BufRead};
    use std::fs::File;

fn main() {
    let file = File::open("file.txt").unwrap();
    for line in BufReader::new(file).lines() {
        println!("{}", line.unwrap());
    }
}

}

fn main() {
    println!("Hello, world!");
}
