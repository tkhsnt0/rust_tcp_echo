use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::{env, str, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    echo_server(addr)?;
    Ok(())
}

fn echo_server(address: &str) -> Result<(), Box<dyn Error>> {
    //リスニングソケットの作成
    let listner = TcpListener::bind(address)?;
    loop {
        //スレッドをブロックし接続待ち受け
        //クライアントからの接続により接続済みソケットが生成される
        let (mut stream, _) = listner.accept()?;
        //スレッドを新たに生成し起動、接続済みソケットは生成したスレッドにmoveされる
        thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                //スレッドをブロックし受信待機
                //受信すると確認応答をクライアントに送信する
                let nbytes = stream.read(&mut buffer).unwrap();
                if nbytes == 0 {
                    //クライアントからの切断要求
                    return;
                }
                print!("{}", str::from_utf8(&buffer[..nbytes]).unwrap());
                //クライアントにデータを送信
                //送信すると確認応答をクライアントから受信する
                stream.write_all(&buffer[..nbytes]).unwrap();
            }
        });
    }
}
