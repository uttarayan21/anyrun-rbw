use crate::{Error, Result};
use error_stack::ResultExt;

use std::io::{BufRead as _, Write as _};

pub struct Sock(std::os::unix::net::UnixStream);

impl Sock {
    // not returning anyhow::Result here because we want to be able to handle
    // specific kinds of std::io::Results differently
    pub fn connect() -> std::io::Result<Self> {
        Ok(Self(std::os::unix::net::UnixStream::connect(
            rbw::dirs::socket_file(),
        )?))
    }

    pub fn send(&mut self, msg: &rbw::protocol::Request) -> Result<()> {
        let Self(sock) = self;
        sock.write_all(
            serde_json::to_string(msg)
                .change_context(Error)
                .attach_printable("failed to serialize message to agent")?
                .as_bytes(),
        )
        .change_context(Error)
        .attach_printable("failed to send message to agent")?;
        sock.write_all(b"\n")
            .change_context(Error)
            .attach_printable("failed to send message to agent")?;
        Ok(())
    }

    pub fn recv(&mut self) -> Result<rbw::protocol::Response> {
        let Self(sock) = self;
        let mut buf = std::io::BufReader::new(sock);
        let mut line = String::new();
        buf.read_line(&mut line)
            .change_context(Error)
            .attach_printable("failed to read message from agent")?;
        serde_json::from_str(&line)
            .change_context(Error)
            .attach_printable("failed to parse message from agent")
    }
}
