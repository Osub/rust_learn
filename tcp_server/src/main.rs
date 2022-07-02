use std::net::{TcpListener, Shutdown, TcpStream};
use std::io::{Read, Write};
use std::thread;
fn main() {
    // 监听 8080 端口
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    //遍历连接的客户端
    for stream in listener.incoming() {
        //匹配结果
        match stream {
            Ok(_stream) => {
                thread::spawn(move|| {
                    let mut _stream:TcpStream = _stream;
                    //指定缓冲区大小
                    let mut data = [0 as u8; 50];
                    //读取接收到的客户端信息
                    while match _stream.read(&mut data) {
                        Ok(size) => {
                            println!("接收到的信息 {}", _stream.write(&data[0..size]).unwrap());
                            true
                        },
                        Err(_) => {
                            println!("An error occurred, terminating connection with {}", _stream.peer_addr().unwrap());
                            _stream.shutdown(Shutdown::Both).unwrap();
                            false
                        }
                    } {}
                });
            }
            //错误输出
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
