use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::VecDeque;

struct Redish {
    pub messages : VecDeque<String>,
}

impl Redish {
    fn handle_client(&mut self, mut stream: TcpStream) {
        // ...
        println!("Connected");
        let mut data = String::new();

        // clone stream
        match stream.try_clone() {
            Ok(cloned_stream) => {
                let mut buffer = BufReader::new(cloned_stream);

                match buffer.read_line(&mut data) {
                    Ok(_data_size) => {
                        let vec: Vec<String> = data.split(" ").map(|s| s.to_string()).collect();
                        match vec.get(0) {
                            Some(action) => {
                                println!("Command received: {}", action);

                                if action == "Get" {
                                    println!("Get Something");
                                    match self.messages.get(0) {
                                        Some(elem) => {
                                            match stream.write(elem.as_bytes()) {
                                                Ok(_value) => {
                                                    // success do nothing
                                                }
                                                Err(_e) => {
                                                    return;
                                                }
                                            }
                                        }
                                        None => {
                                            return;
                                        }
                                    }
                                }
                                else if action == "Put" {
                                    let str2put = vec.get(1);
                                    match str2put {
                                        Some(str2put) => {
                                            let put2vec = str2put.to_string();
                                            println!("Put {}", put2vec);
                                            self.messages.push_back(put2vec);
                                        }
                                        None => {

                                        }
                                    }
                                }
                            }
                            None => {
                            }
                        }
                    }
                    Err(_e) => {
                        return;
                    }
                }
            }
            Err(_e) => {
                return;
            }
        }
        println!("Messages Length: {}", self.messages.len());
    }

    fn new() -> Self {
        Redish {
            messages: VecDeque::default()
        }
    }
}

fn main() {
    let mut redish = Redish::new();
    let listener = TcpListener::bind("127.0.0.1:10000");
    match listener {
        Ok(listener) => {
             //for stream in redish.listener.incoming() {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        redish.handle_client(stream);
                    }
                    Err(_e) => {
                        continue;
                    }
                }
            }
        }
        Err(_e) => {
            return;
        }
    }
}
