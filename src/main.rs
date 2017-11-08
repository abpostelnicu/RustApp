use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

struct Redish {
    pub listener: TcpListener,
    pub messages : VecDeque<String>,
}

impl Redish {
    fn handle_client(&mut self, mut stream: TcpStream) {
        // ...
        println!("Connected");
        println!("Messages Length: {}", self.messages.len());
        let mut data = String::new();
        let mut buffer = BufReader::new(stream.try_clone().unwrap());
        buffer.read_line(&mut data);
        println!("Command received: {}", data);
        //

        if (data == "Get") {
            println!("Get Something");
            let elem = self.messages.get(0).unwrap();

            stream.write(elem.as_bytes());
        }
        else {
            let vec: Vec<String> = data.split(" ").map(|s| s.to_string()).collect();
            if vec.len() == 2 && vec.get(0).unwrap() == "Put" {
                let str2put = vec.get(1).unwrap().to_string();
                println!("Put {}", str2put);
                self.messages.push_back(str2put);
            }
        }
        //else
    }

    fn new(bind_to: String) -> Self {
        Redish {
            listener: TcpListener::bind(bind_to).unwrap(),
            messages: VecDeque::default()
        }
    }
}

fn main() {
    //let mut redish = Redish::new(String::from("127.0.0.1:8989"));
    let mut redish = Redish::new(String::from("127.0.0.1:0000"));
    let mut listener = TcpListener::bind("127.0.0.1:10000").unwrap();
    //for stream in redish.listener.incoming() {
    for stream in listener.incoming() {
        redish.handle_client(stream.unwrap());
    }
}
