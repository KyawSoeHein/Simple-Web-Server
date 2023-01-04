
pub struct Server {
    address : String 
}

impl Server {
    pub fn new(addr : String) -> Server {
        return Server { address: addr };
    }

    pub fn run(self){
        println!("Listening on {}", self.address);
    }
}