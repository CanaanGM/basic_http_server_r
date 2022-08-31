use std::{net::TcpListener, io::Read, io::Write};
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
pub struct Server {
    addr: String,
}

    impl Server {
    // 
    pub fn new(addr:String) -> Self{
        Server { addr }
    }
    // methods needs "self"
    pub fn run(self) {
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
                                Response::new(
                                    StatusCode::OK,
                              Some("<h1>It Lives!!</h1>".to_string()),
                            )
                            },

                            Err(err) =>  {
                                println!("Failes on: {}",err);
                                Response::new(StatusCode::BadRequest, None)
                            }
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
    