use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;
use std::io;


/*###############################################################################
  #                     read_file                                               #
  #                                                                             #
  # Function input:                                                             #
  # data -> String: path to the file being read                                 #
  #                                                                             #
  # Funcion output:                                                             #
  # hash -> HashMap<String,f64> : hashmap with the collected info               #
  #         ['triplete','counter']                                              #
  #                                                                             #
  # What the funcion does:                                                      #
  # Reads the file and stores the data into an hashmap                          #
  ###############################################################################*/
fn read_file(data: &str) -> HashMap<String,f64> {

    println!("Reading file {}",data);

    let  file = File::open(data);
    assert!(file.is_ok(), "Couldn't open file");

    let buf = BufReader::new(file.unwrap());
    let mut hash = HashMap::new();
    let mut split_1 = String::new();
    let mut split_2 = String::new();
    let mut total=0.0;
    for line in buf.lines() {

        assert!(line.is_ok(), "falhou a ler linha");
        let linha = line.unwrap();
        let mut i = 0;
        //println!("{}", linha.chars().collect::<Vec<char>>().len());
        for letra in linha.chars() {
            match i {
                0 ... 2 => split_1.push(letra),
                3 => {},
                _ => split_2.push(letra),
            }

            i+=1;
        }
        total=total+split_2.parse::<f64>().unwrap();
        hash.insert(split_1.to_string(),split_2.parse::<f64>().unwrap());
        split_1.clear();
        split_2.clear();

    }

    for (__,values) in hash.iter_mut(){
        *values=*values/total;

    }
    hash
}

/*###############################################################################
  #                     create_trip                                             #
  #                                                                             #
  # Function input:                                                             #
  # input -> String: string to be divided into triplets                         #
  #                                                                             #
  # Funcion output:                                                             #
  # result -> Vec<String> :Vec containing all triplets (triplets are strings)   #
  #                                                                             #
  # What the funcion does:                                                      #
  # Separetes the inputed string into triplets                                  #
  ###############################################################################*/
fn create_trip (input: &str) -> Vec<String> {

    let input_vec  = input.chars().collect::<Vec<char>>().clone();
    let x = input_vec.len();
    if x<3 {
        println!("Please insert a bigger sentence");
        let result = Vec::with_capacity(0);
        return result;
    }

    let capacity = x - 2;
    let mut result = Vec::with_capacity(capacity);

    for i in (0 .. capacity){
        result.push(String::new());
        result[i].push(input_vec[i]);
        result[i].push(input_vec[i+1]);
        result[i].push(input_vec[i+2]);
        println!("{}",result[i]);

    }
    result
}

/*###############################################################################
  #                     calc_stat                                               #
  #                                                                             #
  # Function input:                                                             #
  # input -> &Vec<String> : reference of the vec containing the triplets        #
  # hash ->  &HashMap<String,f64>: Reference to the hashmap containing the      #
  #          language date                                                      #
  # Funcion output:                                                             #
  # total -> f64 : result of the processing of the triplets                     #
  #                                                                             #
  # What the funcion does:                                                      #
  # For the language with the data in the hash map calculates the probability   #
  # of being the given language for the inputed tripletes                       #
  ###############################################################################*/
fn calc_stat(input: &Vec<String>, hash: &HashMap<String,f64>) -> f64 {

    let mut total=1.0;

    for i in input {
        if hash.contains_key(i) {
            total=hash[i]*total;
            //println!("total:{}",total);
        }
    }
total
}


fn main() {
    let mut program_input =String::new();
    let pt= read_file("/home/rami/cool_stuff/Lang_detector_rust/src/pt_trigram_count_pruned_100000.tsv");
    let fr= read_file("/home/rami/cool_stuff/Lang_detector_rust/src/fr_trigram_count_pruned_100000.tsv");
    let es= read_file("/home/rami/cool_stuff/Lang_detector_rust/src/es_trigram_count_pruned_100000.tsv");
    let mut pt_total=0.0;
    let mut es_total=0.0;
    let mut fr_total=0.0;
    loop {
        println!("Insert here the phrase:");

        io::stdin().read_line(&mut program_input);
        let data=create_trip(&program_input);

        if data.len()>0 {
            pt_total=calc_stat(&data,&pt);
            fr_total=calc_stat(&data,&fr);
            es_total=calc_stat(&data,&es);

            if pt_total>fr_total && pt_total > es_total {
                println!("Portugues");
            }
            else if fr_total>pt_total && fr_total > es_total {
                println!("Frances");
            }
            else if es_total > fr_total && es_total>pt_total {
                println!("Espanhol");
            }
        }

    }
}
