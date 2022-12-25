#![allow(dead_code)]
#![allow(unused_variables)]

use bytes::BytesMut;
use tokio::{io::{BufWriter, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use virus::codec::{Decoder, Encoder};

use crate::{frame::{Frame, FrameCodec}, error::VirusError};

#[derive(Debug)]
pub struct Connection {
    // stream provides write level buffering.
    stream: BufWriter<TcpStream>,

    // the buffer for reading frames.
    inbound_buffer: BytesMut,

    // the buffer for writing frames.
    outbound_buffer: BytesMut,

    // encode and decode frame.
    codec: FrameCodec,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`.
    /// Read and write bufffers are initialized.
    pub fn new(socket: TcpStream) -> Self {
        Connection {
            stream: BufWriter::new(socket),
            inbound_buffer: BytesMut::with_capacity(4 * 1024),
            outbound_buffer: BytesMut::with_capacity(4 * 1024),
            codec: FrameCodec::default(),
        }
    }

    /// Read s single `Frame` value from the underlying stream.
    /// 
    /// The function waits until it has retrieved enough data to parse a frame.
    /// Any data remaining in the read buffer after the frame has been parsed is
    /// kept there for the next call to `read frame`.
    /// 
    /// # Returns
    /// 
    /// On success, the receiveed frame is returned. If the `TcpStream` is closed
    /// in a way that doesn't break a frame in half, it returns `None`.
    /// Otherwise, an error is returned.
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, VirusError> {
        loop {
            // Attempt to parse a frame frame the bufferd data. 
            // If enough data has been buffered, the frame is returned.
            if let Some(frame) = self.codec.decode(&mut self.inbound_buffer)? {
                return Ok(Some(frame));
            }

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. 
            // `0` indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.inbound_buffer).await? {
                // The remote closed the connection. For this to be a clean shutdown,
                // there should be no data in the read buffer. 
                // If there is, this means that the peer closed the socket while
                // sending a frame.
                if self.inbound_buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    /// Write a single `Frame` value to the underlying stream.
    /// 
    /// The `Frame` value is written to the socket using write_* funcitons
    /// providec by `AsyncWrite`. Calling these functions directly on 
    /// a `TcpStream` is **not** advised, as this will result in a large number of
    /// syscalls. However, it's fine to call these functions on a *buffered*
    /// write stream. The data will be written to the buffer. 
    /// Once the buffer is full, it's flushed to the undlying socket.
    pub async fn write_frame(&mut self, frame: Frame) -> Result<(), VirusError> {
        self.codec.encode(frame, &mut self.outbound_buffer)?;
        self.stream.write_all_buf(&mut self.outbound_buffer).await?;
        Ok(())
    }

    /// Shutdown the output stream.
    pub async fn shutdown(&mut self) -> Result<(), VirusError> {
        self.stream.shutdown().await?;
        Ok(())
    }
}
