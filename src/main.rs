use std::{net::{TcpStream, TcpListener}};
//引入标准库net模块的 TcpStream,TcpListener struct
use std::io::prelude::*; 
//引入io模块prelude 倒入全部模块
fn main() { 
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //绑定端口，创建一个监听者，同时使用unwrap 隐式处理panic
    for stream in listener.incoming(){
        //调用incoming方法返回 iterator 接收的信息
        match stream{
            // 匹配stream
            Ok(stream)=>{ 
                //ok 没有错误执行handle方法
                 handle_clinet(stream);
            }
            Err(e)=>{
                //报错 panic
                panic!("connection failed{}",e);
            }
        }
    }
}

/// 处理流
fn handle_clinet(mut stream:TcpStream){
    //接收可变参数类型 TcpStream
    let mut buffer = [0;1024];
    //可变定义1024 空间array
    stream.read(&mut buffer).unwrap();
    //读
    let req_str =String::from_utf8_lossy(&buffer[..]);
    // 定义返回内容,将字符转为str 
    print!("Request:{}",req_str);
    //打印 
    let response = format!("echo>> {}",req_str);
    
    stream.write(response.as_bytes()).unwrap();
    //输出返回 转为字节
    stream.flush().unwrap();
    //清理流
}
