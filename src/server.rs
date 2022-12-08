use std::net::TcpListener;
use tungstenite::accept;

use crate::conn::Conn;

pub struct Server {
    server: TcpListener,
}

impl Server {
    pub fn new(addr: &str) -> Result<Self, std::io::Error>{
	let server = TcpListener::bind(addr).unwrap();
	server.set_nonblocking(true)?;
	Ok(Server {
	    server: server,
	})
    }

    pub fn new_connections(&self) -> Vec<Conn> {
	let mut conns: Vec<Conn> = vec![];
	loop {
	    match self.server.accept() {
		Ok((stream, addr)) => {
		    match accept(stream) {
			Ok(websocket) => {
			    match Conn::new(websocket, addr.to_string()) {
				Ok(conn) => {
 				    conns.push(conn);
				}
				Err(e) => {
				    println!("Warning: Error when trying to create a conn: {}", e);
				}
			    }
			}
			Err(e) => {
			    println!("Warning: Error when trying to validate a connection: {}", e);
			}
		    }
		}
		Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
		    break;
		}
		Err(e) => {
		    println!("Warning: Unexpected error when trying to accept a connection: {}", e);
		    break;
		}
	    }
	}
	return conns;
    }
}
