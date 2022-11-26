use bytes::BytesMut;
use tokio::{io::BufWriter, net::TcpStream};

#[derive(Debug)]
pub struct Connection {
    // stream provides write level buffering.
    stream: BufWriter<TcpStream>,

    // the buffer for reading frames.
    buffer: BytesMut,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
}
