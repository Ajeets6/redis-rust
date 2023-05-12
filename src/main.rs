// Uncomment this block to pass the first stage
use std::net::{TcpListener,TcpStream};
use std::io::{Write,Read};

fn handle_connection(mut _stream: TcpStream) {
  //...
  let mut buf = [0; 256];
  loop{
  let read=_stream.read(&mut buf).unwrap();
  if read==0{break;}
  _stream.write("+PONG\r\n".as_bytes()).unwrap();
  }
}

fn main(){
    //You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Started");

    //Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //listener.set_ttl(100).expect("could not set TTL");
    
    for stream in listener.incoming() {
      
       match stream {
         
            Ok(mut _stream) => {
               println!("accepted new connection");
               handle_connection(_stream);
            }
            
           Err(e) => {
               println!("error: {}", e);
               
           }
       }
       
        
    }
    
    
    
}
