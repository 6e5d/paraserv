use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{BufReader, BufRead};

pub struct ParamServer {
	listener: UnixListener,
	send: Sender<Arc<Vec<f32>>>,
}

impl ParamServer {
	pub fn new(sock_path: &str, send: Sender<Arc<Vec<f32>>>) -> Self {
		let listener = UnixListener::bind(sock_path).unwrap();
		Self {
			listener,
			send,
		}
	}

	fn proc_client(&self, stream: std::io::Result<UnixStream>) -> std::io::Result<()> {
		let e = Err(std::io::ErrorKind::InvalidData.into());
		let stream = stream?;
		let mut buf = String::new();
		let mut br = BufReader::new(stream);
		loop {
			buf.clear();
			let size = br.read_line(&mut buf)?;
			eprint!("recv: {}", buf);
			if size == 0 { return Ok(()) }
			let mut p: Vec<f32> = Vec::new();
			for num in buf.trim_end().split_whitespace() {
				if let Ok(v) = num.parse::<f32>() {
					p.push(v);
				} else {
					return e;
				}
			}
			let p = Arc::new(p);
			self.send.send(p).unwrap();
		}
	}

	pub fn run(self) {
		for stream in self.listener.incoming() {
			if let Err(e) = self.proc_client(stream) {
				eprintln!("{:?}", e);
			}
		}
	}
}
