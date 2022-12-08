use std::net::TcpStream;
use tungstenite::WebSocket;
use tungstenite::protocol::Message;

pub type ConnId = String; // TODO: change to ID from browser

pub struct Conn {
    websocket: WebSocket<TcpStream>,
    id: ConnId,
    dead: bool,
}

impl Conn {

    pub fn new(mut ws: WebSocket<TcpStream>, addr: String) -> Result<Self, std::io::Error> {
	ws.get_mut().set_nonblocking(true)?;
	Ok(Conn {
	    websocket: ws,
	    id: addr,
	    dead: false,
	})
    }

    pub fn id(&self) -> ConnId {
	self.id.clone()
    }

    pub fn is_dead_and_empty(&self) -> bool {
	self.dead
    }

    pub fn get_recved_msgs(&mut self) -> Vec<String> {
	if self.dead {
	    return vec![];
	}
	let mut msgs: Vec<String> = vec![];
	loop {
	    match self.websocket.read_message() {
		Ok(Message::Text(s)) => {
		    msgs.push(s);
		}
		Ok(other) => {
		    println!("Warning: non-string message: {:?}", other);
		}
		Err(tungstenite::error::Error::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
		    break;
		}
		Err(e) => {
		    println!("Conn {} is dying because: <{}>", self.id, e); // TODO: die
		    self.dead = true;
		    break;
		}
	    }
	}
	/*
	while let Ok(msg) = self.websocket.read_message() {
	    if let Message::Text(s) = msg {
		msgs.push(s);
	    } else {
		println!("Warning: non-string message: {:?}", msg);
	    }
	}
*/
	msgs
    }

    pub fn send_msg(&self, msg: String) {
	println!("can't send {}", msg);
    }

}
