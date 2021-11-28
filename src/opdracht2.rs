use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub fn run() {

    let mut file = File::open("input2.txt").expect("Foutje");

    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .expect("Foutje2");

    //println!("File contents: \n \n {}", contents);    werkt

    contents = contents.to_string();

    let vect: Vec<&str> = contents.lines().collect();

    for i in 0..vect.len() {

        let x = vect[i];

        let chars: Vec<char> = x.chars().collect();

        //println!("{:?}", chars);

        let mut a = chars[0] as i32 - 0x30;
        let b = chars[1];
        let mut c = 0;
        let mut d = ' ';

        let mut tweegetallen = false;

        if b == '-' {
            c = chars[2] as i32 - 0x30; 
            d = chars[3];
        }
        else{
            a *= 10;
            a += b as i32 - 0x30;
            c = chars[3] as i32 - 0x30;
            d = chars[4];
            tweegetallen = true;
        }

        let mut kar: char = ' ';

        if d == ' ' {
            if tweegetallen == false {
            kar = chars[4];
            } 
            else {
                kar = chars[5];
            }
        }

        else {
            c *= 10;
            c += d as i32 - 0x30;

            if tweegetallen == false {
                kar = chars[5];
            }
            else {
                kar = chars[6];
            }
        }


        println!("{}-{}: {}", a, c, kar);

        let min = a;
        let max = c;
    }
}

