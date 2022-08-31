use std::{net::TcpListener, io::Read, io::Write};
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response{
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest,None)
    }
}
pub struct Server {
    addr: String,
}

    impl Server {
    // 
    pub fn new(addr:String) -> Self{
        Server { addr }
    }
    // methods needs "self"
    pub fn run(self, mut handler:impl Handler) {
        println!("Listining on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
         loop {

            match listener.accept(){
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                  match  stream.read(&mut buffer){
                    Ok(_) => {
                        println!("recieved a request: {}", String::from_utf8_lossy(&buffer));
                        // a slice [..];
                        let response = match Request::try_from(&buffer[..]){

                            Ok(request)=> {
                               handler.handle_request(&request)
                            },

                            Err(err) => 
                                handler.handle_bad_request(&err)
                        } ;
                        
                        if let Err(err) = response.send(&mut stream){
                            println!("Fails on: {}",err)

                        }

                    },
                    Err(err) => println!("Failed to read from connection: {}", err)
                  }
                },
                Err(err) => println!("failed: {}",err)
                
            }
        }
    }
    
}
    