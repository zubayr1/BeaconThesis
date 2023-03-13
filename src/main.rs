//imports
use std::thread;
use std::io;
use std::fs::File;
use std::io::{ prelude::*, BufReader};
//extern
extern crate rand;
use rand::Rng;

//import own files
mod node1;
mod node2;
mod node3;
mod node4;


fn run_nodes()
{
    let mut pubkeys: Vec<String> = Vec::new();

    let nodesfile = File::open("./nodes_information.txt").expect("cant open the file");
    let reader = BufReader::new(nodesfile);
    

    for line in reader.lines() {
        let line_uw = line.unwrap();
        
        let pubkey = line_uw.split("-");

        let mut count=0;
        for db in pubkey {
            count+=1;

            if count==3
            {
                pubkeys.push(db.to_string());
                count=0;
            }
            
      }
    }

    let mut pks1 = pubkeys.clone();
    let mut pks2 = pubkeys.clone();
    let mut pks3 = pubkeys.clone();
    let mut pks4 = pubkeys.clone();

    let random_number = rand::thread_rng().gen_range(1 .. 1000);

    let handle1 = thread::spawn( move || {
        
        
        for iter in 1..2
        {
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;

            println!("a{}",iter);
            
            node1::initiate_node1(random_number+iter, node_port_start, &mut pks1);
        }
        

    });

    let handle2 = thread::spawn(move || {

        for iter in 1..2
        {
            println!("b{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;

            node2::initiate_node2(random_number+iter, node_port_start, &mut pks2);
        }
        

    });

    let handle3 = thread::spawn(move || {

        for iter in 1..2
        {
            println!("c{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;
           
            node3::initiate_node3(random_number+iter, node_port_start, &mut pks3);
        }
        

    });


    let handle4 = thread::spawn(move || {
        
        for iter in 1..2
        {
            println!("d{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;
           
            node4::initiate_node4(random_number+iter, node_port_start, &mut pks4);
        }
        

    });


    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    
}


fn create_keys()
{
    node1::create_keys();
    node2::create_keys();
    node3::create_keys();
    node4::create_keys();
}

fn main() 
{
    println!("Starting");    
    

    let mut input = String::new();
    let keys: &str = "keys";

    println!("execution type");

    match io::stdin().read_line(&mut input)
    {
        
        Ok(_) => {
            if input.trim() == keys
            {
                create_keys();
            }
            else 
            {
                run_nodes();
            }
        },
        Err(e) => {
            println!("error {}", e);
        }
    }

    
    
}
