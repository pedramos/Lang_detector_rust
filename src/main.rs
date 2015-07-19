fn read_file(data: &std){

    println!("Reading file {}",data);


    //CENAS sacadas da documentação. Falta adaptar
    let mut f = try!(File::open("foo.txt"));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    assert_eq!(s, "Hello, world!");

}

fn main() {
    println!("Hello, world!");
}
