extern crate mio;
extern crate bincode;
extern crate rustc_serialize;

use std::thread;
use mio::*;
use mio::deprecated::{EventLoop, Handler};
use mio::udp::*;
use std::net;
use std::io;

mod packet;
use packet::Packet;

const SERVER: Token = Token(10_000_000);

static mut packetCounter: u32 = 0;

pub struct UdpHandler {
    rx: UdpSocket,
}

impl UdpHandler {
    fn new(rx: UdpSocket) -> UdpHandler {
        UdpHandler {
            rx: rx,
        }
    }
}

impl Handler for UdpHandler {
    type Timeout = ();
    type Message = u32;

    fn ready(&mut self, event_loop: &mut EventLoop<UdpHandler>, token: Token, events: Ready) {

           if events.is_readable() {
               match token {
                   SERVER => {
                       let mut buf: [u8; 1472] = [0; 1472];

                       let received = self.rx.recv_from(&mut buf);//.unwrap().unwrap();
                       println!("Received datagram...");

                       if let Some((size, sock)) = received.unwrap() {//.unwrap();

                            let addr = Some(sock);
                            //println!("bytes: {:?} from: {:?}", size, addr);

                            let data = Vec::from(&buf[0..size]);

                            let decoded: Packet = bincode::rustc_serialize::decode(&data[..]).unwrap();

                            unsafe {packetCounter+=1;

                            println!("{}", packetCounter);}

                           //println!("We are receiving a datagram now...");
                          // println!("Packet: {:?}", decoded);
                          // event_loop.shutdown();
                       }
                   },
                   _ => ()
               }
           }

           if events.is_writable() {
               println!("Event is writable...");
           }
       }

    fn notify(&mut self, event_loop: &mut EventLoop<UdpHandler>, msg: u32) {
        println!("Message notify received: {}", msg);
        event_loop.shutdown();
    }
}


fn socket(listen_on: net::SocketAddr) -> mio::udp::UdpSocket {
  //let attempt = net::UdpSocket::bind(listen_on);
  let attempt = mio::udp::UdpSocket::bind(&listen_on);
  let socket;
  match attempt {
    Ok(sock) => {
      println!("Bound socket to {}", listen_on);
      socket = sock;
    },
    Err(err) => panic!("Could not bind: {}", err)
  }
  socket
}

pub fn main() {
    let mut event_loop = EventLoop::new().unwrap();

    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, 8890);
    let skt = socket(net::SocketAddr::V4(listen_addr));

    event_loop.register(&skt, SERVER, Ready::readable(), PollOpt::edge()).unwrap();

    let _ = event_loop.run(&mut UdpHandler::new(skt));
}
