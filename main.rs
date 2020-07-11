use std::env;
use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::str;
use std::process;

static HELP_MSG: &'static str = 
"USAGE:
   rust64                  base64 encode stdin
   rust64 -d, --decode     base64 decode stdin
   rust64 -h, --help       print this message!
   
EXAMPLES:
   cat <file> | rust64
   echo <base64_string> | rust64 -d\n";

macro_rules! error_message {
    ($input:expr) => {
        println!("{}", $input);
        process::exit(1);
    }
}

fn encode_hashmap() -> HashMap<&'static str, &'static str> {
    [
        ("000000","A"), ("010000","Q"), ("100000","g"), ("110000","w"),
        ("000001","B"), ("010001","R"), ("100001","h"), ("110001","x"),
        ("000010","C"), ("010010","S"), ("100010","i"), ("110010","y"),
        ("000011","D"), ("010011","T"), ("100011","j"), ("110011","z"),
        ("000100","E"), ("010100","U"), ("100100","k"), ("110100","0"),
        ("000101","F"), ("010101","V"), ("100101","l"), ("110101","1"),
        ("000110","G"), ("010110","W"), ("100110","m"), ("110110","2"),
        ("000111","H"), ("010111","X"), ("100111","n"), ("110111","3"),
        ("001000","I"), ("011000","Y"), ("101000","o"), ("111000","4"),
        ("001001","J"), ("011001","Z"), ("101001","p"), ("111001","5"),
        ("001010","K"), ("011010","a"), ("101010","q"), ("111010","6"),
        ("001011","L"), ("011011","b"), ("101011","r"), ("111011","7"),
        ("001100","M"), ("011100","c"), ("101100","s"), ("111100","8"),
        ("001101","N"), ("011101","d"), ("101101","t"), ("111101","9"),
        ("001110","O"), ("011110","e"), ("101110","u"), ("111110","+"),
        ("001111","P"), ("011111","f"), ("101111","v"), ("111111","/")
    ].iter().cloned().collect()
}

fn decode_hashmap() -> HashMap<&'static str, &'static str> {
    [
        ("A","000000"), ("Q","010000"), ("g","100000"), ("w","110000"),
        ("B","000001"), ("R","010001"), ("h","100001"), ("x","110001"),
        ("C","000010"), ("S","010010"), ("i","100010"), ("y","110010"),
        ("D","000011"), ("T","010011"), ("j","100011"), ("z","110011"),
        ("E","000100"), ("U","010100"), ("k","100100"), ("0","110100"),
        ("F","000101"), ("V","010101"), ("l","100101"), ("1","110101"),
        ("G","000110"), ("W","010110"), ("m","100110"), ("2","110110"),
        ("H","000111"), ("X","010111"), ("n","100111"), ("3","110111"),
        ("I","001000"), ("Y","011000"), ("o","101000"), ("4","111000"),
        ("J","001001"), ("Z","011001"), ("p","101001"), ("5","111001"),
        ("K","001010"), ("a","011010"), ("q","101010"), ("6","111010"),
        ("L","001011"), ("b","011011"), ("r","101011"), ("7","111011"),
        ("M","001100"), ("c","011100"), ("s","101100"), ("8","111100"),
        ("N","001101"), ("d","011101"), ("t","101101"), ("9","111101"),
        ("O","001110"), ("e","011110"), ("u","101110"), ("+","111110"),
        ("P","001111"), ("f","011111"), ("v","101111"), ("/","111111")
    ].iter().cloned().collect()
}

fn valid_chars_hashmap() -> HashMap<u8, &'static str> {
    [
        (65,"A"), (81,"Q"),  (103,"g"), (119,"w"),
        (66,"B"), (82,"R"),  (104,"h"), (120,"x"),
        (67,"C"), (83,"S"),  (105,"i"), (121,"y"),
        (68,"D"), (84,"T"),  (106,"j"), (122,"z"),
        (69,"E"), (85,"U"),  (107,"k"), (48,"0"),
        (70,"F"), (86,"V"),  (108,"l"), (49,"1"),
        (71,"G"), (87,"W"),  (109,"m"), (50,"2"),
        (72,"H"), (88,"X"),  (110,"n"), (51,"3"),
        (73,"I"), (89,"Y"),  (111,"o"), (52,"4"),
        (74,"J"), (90,"Z"),  (112,"p"), (53,"5"),
        (75,"K"), (97,"a"),  (113,"q"), (54,"6"),
        (76,"L"), (98,"b"),  (114,"r"), (55,"7"),
        (77,"M"), (99,"c"),  (115,"s"), (56,"8"),
        (78,"N"), (100,"d"), (116,"t"), (57,"9"),
        (79,"O"), (101,"e"), (117,"u"), (43,"+"),
        (80,"P"), (102,"f"), (118,"v"), (47,"/")
    ].iter().cloned().collect()
}

fn retrieve_input() -> Vec<u8> {
    // create vector of u8 ints to hold bytes of stdin and fill it
    let mut input_buff: Vec<u8> = Vec::new();
    
    // unwrap takes each byte in stdin and reveals the T value or panics
    for i in io::stdin().bytes() {
        let byte = i.unwrap();
        input_buff.push(byte);
    }

    input_buff
}

fn encode_stringify(input: Vec<u8>) -> String {
    // just create a string thats the binary representation of the input buffer
    let mut octet_string = String::new();
    for i in 0..input.len() {
        let byte = format!("{:08b}", input[i]);
        octet_string.push_str(&byte);
    }

    octet_string
}

fn base64_encode(input: &mut String) {
    // we get our pretty hashmap :)
    let b64_table = encode_hashmap();

    // pad our input string so that it's evenly divisible by 6
    while input.len() % 6 != 0 {
        input.push_str("0");
    }

    // create our translation string
    let mut translation = String::new();

    // iterate through our input string one sextet at a time matching values
    // for each sextet with the base64 table
    let iterations = input.len() / 6;
    for i in 0..iterations {
        let current_slice = &input[i*6..(i+1)*6];
        match b64_table.get(current_slice) {
            Some(found_value) => translation.push_str(found_value),
            None => {
                let message = format!("Couldn't match value {}", current_slice);
                error_message!(message);
            }
        };
    }

    // add padding if our encoded output is not divisible by 4
    while translation.len() % 4 != 0 {
        translation.push('=');
    }
    
    println!("{}", translation);
}

fn decode_stringify(mut input: Vec<u8>) -> String {
    // iterate over the input vector and remove newlines (10)
    input.retain(|&x| x != 10);
    
    // base64 with padding should always be divisible by 4
    if input.len() % 4 != 0 {
        println!("Input length was: {}", input.len());
        error_message!("Invalid base64 detected, possibly a padding issue.");
    }

    // iterate over the input vector and remove padding (=)
    // slower than necessary since we start at the beginning which will never
    // have padding, but this performance isn't critical obviously
    input.retain(|&x| x != 61);

    // make sure every character is legal
    let valid_table = valid_chars_hashmap();
    for i in input.iter() {
        match valid_table.get(i) {
            None => {
                error_message!("Invalid base64 character detected.");
            }
            _ => (),
        };
    }

    // start building our binary string "000001100010..."
    let mut octet_string = String::new();
    let b64_table = decode_hashmap();
    for i in input.iter() {
        // unwrap is fine here since there's no way it's an illegal char
        let val = valid_table.get(i).unwrap();

        // lookup the value which should be something like "A" in b64_table
        let append_val = b64_table.get(val).unwrap();

        // add to string
        octet_string.push_str(append_val);
    }
    
    octet_string
}

fn base64_decode(input: &mut String) {
    // trim sextet padding off
    while input.len() % 8 != 0 {
        input.pop();
    }
    
    // create a vector of u8
    let mut decimal_vec = Vec::new();
    let iterations = input.len() / 8;
    for i in 0..iterations {
        let current_slice = &input[i*8..(i+1)*8];
        // converts our 8 digit strings into a real integer u8
        let decimal = u8::from_str_radix(current_slice, 2).unwrap();
        decimal_vec.push(decimal);
    }

    // write all the bytes to stdout without any regard for encoding 
    match io::stdout().write_all(&decimal_vec) {
        Ok(()) => (),
        Err(e) => {
            error_message!(e);
        }
    }

}

fn encode_routine() {
    // retrieves vector of u8's
    let input_buff = retrieve_input();

    // converts vector of u8's into string binary representation
    // padded to octets
    let mut octet_string = encode_stringify(input_buff);

    // converts string binary representation to base64 encoded string &prints
    base64_encode(&mut octet_string);
}

fn decode_routine() {
    // retrieves vector of u8's
    let input_buff = retrieve_input();
    
    // check to make sure its valid base64 and then create string of binary
    // representation
    let mut octet_string = decode_stringify(input_buff);

    // hopefully it writes the decoded bytes to stdout, that or it dies
    base64_decode(&mut octet_string);
}

fn check_flag(flag: &String) {
    if flag != "-d" && flag != "--decode" {
        println!("{}", HELP_MSG);
    } else {
        decode_routine();
    }
}

fn main() {
    // ["rust64", "--decode"] <-- example vector
    let args: Vec<String> = env::args().collect();

    // check input length, shouldn't be more than 2 inputs
    match args.len() {
        1 => encode_routine(),
        2 => check_flag(&args[1]),
        _ => println!("{}", HELP_MSG),
    };
}
