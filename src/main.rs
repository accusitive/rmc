use minecraft_varint::VarIntRead;
use network::{Packet, ReadMCString};
use std::{
    io::Cursor,
    net::{TcpListener, TcpStream},
};
type CursorType<'a> = Cursor<&'a mut [u8]>;
mod network;
mod packets;
use packets::*;
fn handle(stream: &mut TcpStream) {
    let mut state = 0;
    let mut buffer = [0; 2048];
    let mut is_running = true;

    while is_running {
        let read = std::io::Read::read(stream, &mut buffer).unwrap();
        if read == 0 {
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            is_running = false;
        }
        let mut cursor = Cursor::new(&mut buffer[..]);
        let length = cursor.read_var_i32().unwrap();
        let id = cursor.read_var_i32().unwrap();
        println!("State {:#?}\nLength: {:#?}\nId: {:#x?}", state, length, id);
        if state == 0 {
            if id == 0 {
                let hs = HandshakeC2S::read(&mut cursor).unwrap();
                state = hs.next_state;
                println!("Hs {:#?}", hs);
            }
        }
        // Login
        if state == 2 {
            if id == 0 {
                let player_name = LoginStartC2S::read(&mut cursor).unwrap();
                
                println!("{:#?}", player_name.name);
                let success = LoginSuccessS2C {
                    username: "User".to_string(),
                    uuid: 0
                };
                success.write(&mut cursor);
            }
        }
    }
    println!("Let is done")
}
fn server() {
    println!("Server started");
    let mut handles = vec![];
    let socket = TcpListener::bind("127.0.0.1:2500").unwrap();
    for (mut stream, socket_addr) in socket.accept() {
        handles.push(std::thread::spawn(move || {
            handle(&mut stream);
        }));
    }
}
fn main() {
    loop {
        println!("starting a server...");
        server();
        println!("Server ended...")
    }
    // println!("Hello, world!");
}