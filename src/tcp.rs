use std::error::Error;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

/// Sends the data to the TCP server and returns the response.
pub fn write(addr: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr)?;

    let to_sent = data.to_string() + "\n";
    stream.write_all(to_sent.as_bytes())?;

    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    reader.read_line(&mut line)?;

    Ok(line.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use crate::write;

    #[test]
    fn test_write() {
        let sent = "hello";
        let got = write("echo.bora.sh:1337", sent).unwrap();
        if got == sent {
            return;
        } else {
            assert_eq!(got, sent.to_uppercase());
        }
    }
}
