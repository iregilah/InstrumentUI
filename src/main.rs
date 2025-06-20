use tokio_lxi::LxiDevice;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), tokio_lxi::Error> {
    // Define the LXI instrument address (IP and port).
    let instrument_ip = "169.254.50.23:5555";
    let addr: SocketAddr = instrument_ip.parse().expect("Invalid IP:Port format");

    // Connect to the oscilloscope using LXI (TCP port 5025).
    let mut device = LxiDevice::connect(&addr).await?;
    println!("Connected to oscilloscope at {}.", instrument_ip);

    // Send the SCPI command to enable Channel 1 (CH1).
    // SCPI command format from Rigol manual: :CHANnel1:DISPlay ON
    device.send(":CHAN1:DISP ON").await?;  // Enable CH1 (turn it ON/display)
    println!("Command sent: CH1 display ON");

    // (Optional) You could verify by querying an identifier or channel status:
    // device.send(\"*IDN?\").await?;
    // let idn_response = device.receive().await?;
    // println!(\"*IDN? response: {}\", idn_response);

    // Drop the device (connection) which will close the TCP connection.
    // The program will exit, leaving CH1 enabled on the oscilloscope.
    Ok(())
}