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
            println!("Listining on {}", self.addr)
        }
        
    }
    