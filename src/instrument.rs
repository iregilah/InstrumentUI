// instrument.rs (backend)
use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::time::Duration;

pub struct Instrument {
    stream: TcpStream,
}

impl Instrument {
    pub fn connect(addr: &str) -> io::Result<Self> {
        println!("[NET] Attempting to connect to {}", addr);
        let mut stream = TcpStream::connect(addr)?;
        // Optionally, set a timeout for reads to avoid hanging indefinitely
        stream.set_read_timeout(Some(Duration::from_secs(2))).ok();
        println!("[NET] Connection established to {}", addr);
        // Query instrument identification
        let mut instrument = Instrument { stream };
        println!("[NET] Sending *IDN? query");
        instrument.write("*IDN?")?;
        let idn = instrument.read_line()?;
        println!("[NET] Instrument address: {}  —  Connected: {}", addr, idn.trim());
        Ok(instrument)
    }

    pub fn write(&mut self, cmd: &str) -> io::Result<()> {
        // Ensure the command ends with a newline
        let mut cmd_string = cmd.to_string();
        if !cmd_string.ends_with('\n') {
            cmd_string.push('\n');
        }
        print!("SCPI> {}", cmd.trim());
        let result = self.stream.write_all(cmd_string.as_bytes());
        if let Err(ref e) = result {
            println!(" (connection failed: {})", e);
        } else {
            println!();
        }
        result
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        println!("[NET] Reading line from instrument");
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];
        // Read until newline or EOF
        while self.stream.read(&mut byte)? > 0 {
            if byte[0] == b'\n' {
                break;
            }
            buffer.push(byte[0]);
        }
        let line = String::from_utf8_lossy(&buffer).into_owned();
        println!("[NET] Received line: {}", line);
        Ok(line)
    }

    pub fn read_block(&mut self) -> io::Result<Vec<u8>> {
        println!("[NET] Reading binary block from instrument");
        // Read the block header first (e.g. "#9..." format for binary data)
        let mut prefix = [0u8; 2];
        self.stream.read_exact(&mut prefix)?;
        if prefix[0] != b'#' {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid block prefix"));
        }
        let num_digits = (prefix[1] as char)
            .to_digit(10)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid block length digit"))? as usize;
        let mut len_buf = vec![0u8; num_digits];
        self.stream.read_exact(&mut len_buf)?;
        let length_str = String::from_utf8_lossy(&len_buf);
        let total_length: usize = length_str.trim().parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid length: {}", e)))?;
        println!("[NET] Expected binary length: {}", total_length);
        // Read the binary data of the specified length
        let mut data = vec![0u8; total_length];
        self.stream.read_exact(&mut data)?;
        println!("[NET] Binary block received ({} bytes)", data.len());
        // Attempt to read a trailing terminator (e.g. newline) without blocking for too long
        self.stream.set_read_timeout(Some(Duration::from_millis(100))).ok();
        let mut trailing = [0u8; 1];
        if let Ok(1) = self.stream.read(&mut trailing) {
            println!("[NET] Read terminator byte: 0x{:02X}", trailing[0]);
        }
        self.stream.set_read_timeout(None).ok();
        Ok(data)
    }
}
