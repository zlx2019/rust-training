use std::net::{TcpListener, TcpStream};


///
/// 单线程Web服务器
/// 
fn main(){
    // 创建TCP服务，监听 127.0.0.1:7878
    let listen: TcpListener = TcpListener::bind(("127.0.0.1", 7878)).expect("bind address fail");
    println!("listen on {:?}", listen.local_addr().unwrap());
    // 处理客户端连接
    for conn in listen.incoming(){
         let stream: TcpStream = conn.unwrap();
         println!("Connection successful");
    }
}