use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn run() {

    let mut file = File::open("input2.txt").expect("Foutje");

    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .expect("Foutje2");

    //println!("File contents: \n \n {}", contents);    werkt

    contents = contents.to_string();

    let mut valid = 0;
    let mut teller2 = 0;


    let vect: Vec<&str> = contents.lines().collect();

    for i in 0..vect.len() {

        let x = vect[i];

        let chars: Vec<char> = x.chars().collect();

        //println!("{:?}", chars);

        let mut a = chars[0] as i32 - 0x30;
        let b = chars[1];
        let mut c = 0;
        let mut d = ' ';
        let mut start = 0;

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

        let mut kar: char;

        if d == ' ' {
            if tweegetallen == false {
            kar = chars[4];
            start = 6;
            } 
            else {
                kar = chars[5];
                start = 7;
            }
        }

        else {
            c *= 10;
            c += d as i32 - 0x30;

            if tweegetallen == false {
                kar = chars[5];
                start = 7;
            }
            else {
                kar = chars[6];
                start = 8;
            }
        }


        //println!("{}-{}: {}", a, c, kar);

        let min = a;
        let max = c;

        let mut hm: HashMap<char, i32> = HashMap::new();

        for i in start..chars.len() {
            if hm.contains_key(&chars[i]) {
                *hm.get_mut(&chars[i]).unwrap() += 1;
            }

            else {
                hm.insert(chars[i], 1);
            }
        }

        //println!("{:?}", hm);

        if hm.contains_key(&kar) && hm[&kar] <= max && hm[&kar] >= min {
            //println!("{:?}", x);
            valid += 1;
        }



        let mut pos1 = false;
        let mut pos2 = false;

        if chars[start + min as usize] == kar {
            pos1 = true;
        }

        if chars[start + max as usize ] == kar {
            pos2 = true;
        }

        //println!("{} {}", chars[start + min as usize], chars[start + max as usize]);

        if pos1 && !pos2 {
            teller2 += 1;
        }

        if !pos1 && pos2 {
            teller2 += 1;
        }

    }

    println!("{}", valid);
    println!("OPDRACHT 2  ----------------------------------------------");
    println!("{}", teller2);

}

