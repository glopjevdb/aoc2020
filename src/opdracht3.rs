use std::fs;
use std::fs::File;
use std::io::prelude::*;
//use std::collections::HashMap;

fn checkTrees(a: u8, b: u8) -> i64{

    let mut file = File::open("input/input3.txt").expect("Foutje");

    let mut contents: String = String::new();

    file.read_to_string(&mut contents)
        .expect("Foutje2");

    contents = contents.to_string();
    let it = contents.split('\n');

    let vec: Vec<String> = it.map(String::from).collect();

    //println!("{:?}", vec);

    println!("{}", vec.len());

    let mut rij = 0;
    let aantal_kolommen = vec[0].len();
    let mut kolom = 0;
    let mut trees = 0;

    while rij < vec.len() {
        println!("R: {} C: {}", rij, kolom);
        let chars: Vec<char> = vec[rij].chars().collect();
        println!("{}", chars[kolom%aantal_kolommen]);
        if chars[kolom%aantal_kolommen] == '#' {
            trees += 1;
        }
        kolom = (kolom + b as usize) % aantal_kolommen;
        rij += a as usize;
    }

    trees
    

}

pub fn run(){
    let mut answer = 1;
    let vec1: Vec<u8> = vec![1,3,5,7,1];
    let vec2: Vec<u8> = vec![1,1,1,1,2];

    for x in 0..vec1.len(){
        answer *= checkTrees(vec2[x], vec1[x]);
    }

    println!("{}", answer);
}