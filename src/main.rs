//imports
use std::thread;

//extern
extern crate rand;
use rand::Rng;

//import own files
mod node1;
mod node2;
mod node3;
mod node4;



fn main() 
{
    println!("Starting");    
    
    let random_number = rand::thread_rng().gen_range(1, 1000);

    

    let handle1 = thread::spawn( move || {
        

        for iter in 1..2
        {
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;

            println!("a{}",iter);
            
            node1::initiate_node1(random_number+iter, node_port_start);
        }
        

    });

    let handle2 = thread::spawn(move || {
        

        for iter in 1..2
        {
            println!("b{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;

            node2::initiate_node2(random_number+iter, node_port_start);
        }
        

    });

    let handle3 = thread::spawn(move || {
        

        for iter in 1..2
        {
            println!("c{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;
           
            node3::initiate_node3(random_number+iter, node_port_start);
        }
        

    });


    let handle4 = thread::spawn(move || {
        

        for iter in 1..2
        {
            println!("d{}",iter);
            let mut node_port_start: u32 = 3333;
            node_port_start+=1000;
           
            node4::initiate_node4(random_number+iter, node_port_start);
        }
        

    });


    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    

    
    
}
