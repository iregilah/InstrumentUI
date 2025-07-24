// src/cxxqt_object.rs
#[cxx_qt::bridge]
mod qobject {
    extern "RustQt" {
        #[qobject]
        type Rigol = super::RigolRust;

        #[qinvokable]
        fn run_demo(self: &Rigol);
    }

    // (If needed, extern "C++" section for Qt signals/slots can be added here)
}

// Define the Rust side struct for the QObject
#[derive(Default)]
pub struct RigolRust;

// Implement the QObject's invokable method in Rust
impl qobject::Rigol {
    pub fn run_demo(&self) {
        // Spawn a thread to run the backend demo without blocking the UI
        std::thread::spawn(|| {
            use std::io::Write;
            use std::net::TcpStream;
            use std::time::Duration;

            println!("=== BASIC DEMO ===");
            // Attempt to connect to the instrument over network (LXI)
            if let Ok(mut stream) = TcpStream::connect_timeout(
                &"169.254.50.23:5555".parse().unwrap(),
                Duration::from_secs(1))
            {
                // Send SCPI commands sequentially, with delays, mimicking the CLI demo
                stream.write_all(b":CHAN1:DISP OFF\n").ok();
                println!(">> CH1 OFF  – A nyomvonal el kell hogy tűnjön.");
                std::thread::sleep(Duration::from_secs(1));

                stream.write_all(b":CHAN1:DISP ON\n").ok();
                println!(">> CH1 ON   – Nyomvonal visszatér.");
                std::thread::sleep(Duration::from_secs(1));

                stream.write_all(b":CHAN1:SCAL 1.5\n").ok();
                println!(">> CH1 SCALE 1.5V/div");
                std::thread::sleep(Duration::from_secs(1));

                stream.write_all(b":AUTOSCALE\n").ok();
                println!(">> AUTOSCALE – A scope újra beállítja a skálákat.");
                std::thread::sleep(Duration::from_secs(2));
            } else {
                eprintln!("Instrument not found on network. (USB support can be added via rusb)");
            }
            println!("=== BASIC DEMO END ===");
        });
    }
}
