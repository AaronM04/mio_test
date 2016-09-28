extern crate mio;

use std::thread;
use mio::*;
use mio::deprecated::{EventLoop, Handler};
use mio::udp::*;
use std::net;
use std::io;

const SERVER: Token = Token(10_000_000);

pub struct UdpHandler {
    tx: UdpSocket,
    rx: UdpSocket,
    msg: Vec<u8>,
}

impl UdpHandler {
    fn new(tx: UdpSocket, rx: UdpSocket, msg : Vec<u8>) -> UdpHandler {
        UdpHandler {
            tx: tx,
            rx: rx,
            msg: msg,
    //        buf: SliceBuf::wrap(msg.as_bytes()),
    //        rx_buf: RingBuf::new(1024)
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
                       
                       let (size, sock) = received.unwrap().unwrap();
                       /*
                       if recv_result.is_err() {
                            // debug b/c we're returning the error explicitly
                            println!("could not recv_from on {:?}: {:?}", self.rx, recv_result);
                            return
                      }
                       
                      if recv_result.as_ref().unwrap().is_none() {
                         // debug b/c we're returning the error explicitly
                         println!("no return address on recv_from: {:?}", self.rx);
                         
                         return
                        }
                        */
                        let addr = Some(sock);
                        println!("bytes: {:?} from: {:?}", size, addr);
                       
                        let data = Vec::from(&buf[0..size]);

                       
                       println!("We are receiving a datagram now...");
                       println!("Msg: {:?}", data);
                       event_loop.shutdown();
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
    
    
    //let sender = event_loop.channel();

    // Send the notification from another thread
//    thread::spawn(move || {
    //    let _ = sender.send(123);
//    });

    let sk2 = socket(net::SocketAddr::V4(net::SocketAddrV4::new(ip, 8891)));

    let _ = event_loop.run(&mut UdpHandler::new(skt, sk2, vec![1,2,3,4,5]));
}