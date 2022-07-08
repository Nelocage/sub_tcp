use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    //绑定一个TcpListener实例
    let listener = TcpListener::bind("127.0.0.1:7878").expect("无法绑定到套接字");

    let addr = listener
        .local_addr()
        .expect("无法绑定到本机端口");

    println!("监听的端口为 {}", addr.port());

    //listener.incoming 从循环中不断读流
    for connection in listener.incoming() {
        //使用match进行错误处理
        match connection {
            Ok(stream) => {
                //如果正确拿到了值，则新开个线程并建立tcp连接
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => panic!("{}", e),
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    //建立一个缓冲区
    let mut buffer = [0; 1024];
    //在打开流的时候写入数据
    if let Err(_) = stream.write("Hello from Substrate".as_bytes()) {
        return;
    }
    println!("连接已建立");

    //循环读取 并读到流中
    loop {
        // stream.write("\r".as_bytes());
        if let Ok(read) = stream.read(&mut buffer) {
            if read == 0 {
                break;
            }
            if let Err(_) = stream.write(&buffer[0..read]) {
                break;
            }
        } else {
            break;
        }
    }
    println!("连接关闭")
}
