use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

use std::sync::atomic::{AtomicU8, Ordering};

static CHECK: AtomicU8 = AtomicU8::new(0);

fn listen_to_client(mut stream: TcpStream) {
    let msg : &[u8; 16]= b"Hello from node3";
    // while match stream.read(&mut data) {
    //     Ok(size) => {
    //         // echo everything!
    //         stream.write(&msg[0..size-1]).unwrap();
    //         true
    //     },
    //     Err(_) => {
    //         println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
    //         stream.shutdown(Shutdown::Both).unwrap();
    //         false
    //     }
    // } {}

    stream.write(msg).unwrap();
}

fn handle_server(node3_port: u32) {
   // println!("server node3");
    let anycast = String::from("0.0.0.0:");

    let address = [anycast.to_string(), node3_port.to_string()].join("");

    let listener = TcpListener::bind(address).unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server node3 listening on port {}", node3_port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    listen_to_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
    // process::exit(0x0100);
}


fn match_tcp_client(address: String, node_port: u32)
{
    match TcpStream::connect(address) {
        Ok(mut stream) => {
           // println!("Successfully connected to server by node3 in port {}", node_port);

            let msg : &[u8; 16]= b"Hello from node3";

            stream.write(msg).unwrap();
            println!("Sent Hello from node3, awaiting reply...");

            let mut data = [0 as u8; 16]; // using 16 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is echoed");
                        

                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Reply: {} to node3", text);
                        CHECK.store(1, Ordering::Relaxed);

                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                    CHECK.store(0, Ordering::Relaxed);
                    handle_client( node_port);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            
        }
    }
   // println!("Terminated.");
    return;
}


fn handle_client(node_port: u32) {
    if CHECK.load(Ordering::Relaxed)==0 
    {

      //  println!("client node3");
        let localhost = String::from("localhost:");

        match_tcp_client([localhost.to_string(), node_port.to_string()].join(""), node_port);

    }
    


}


// init function
pub fn initiate_node3(random_number: u32, node_port_start: u32) {
    
    if random_number%4==2
    {       
        
        let handle1 = thread::spawn( move || {

            handle_server(node_port_start+1+3);          
            
    
        });
    
        let handle2 = thread::spawn(move || {
            
    
            handle_server(node_port_start+2+3);
            
    
        });

        let handle4 = thread::spawn(move || {
            
    
            handle_server(node_port_start+3+4);
            
    
        });
            
        
        handle1.join().unwrap();
        handle2.join().unwrap();
        handle4.join().unwrap();
        
    }
    else 
    {         

        let handle1 = thread::spawn( move || {

            handle_client(node_port_start+1+3);         
            
    
        });
    
        let handle2 = thread::spawn(move || {
            
    
            handle_client(node_port_start+2+3);
            
    
        });

        let handle4 = thread::spawn(move || {
            
    
            handle_client(node_port_start+3+4);
            
    
        }); 
        
        handle1.join().unwrap();
        handle2.join().unwrap();  
        handle4.join().unwrap(); 
    }
}

