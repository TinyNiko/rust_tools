extern crate serial;
use std::time::{Duration, Instant};
use std::fs::File;
use std::path::Path;
use clap::{Arg, App};
use std::io;
use serial::prelude::*;

const SERIAL_PORT_TO_USE: &str = "/dev/ttyUSB0";
const BAUD_RATE_TO_USE: i32 = 115200;
const FLASH_LOCATION_IN_DECIMAL: i32 = 0;
const FLASH_SIZE: i32 = 4;
const TEMPFILEPATH: &str = "/tmp/dumptemp.txt";
const FINALFILEPATH: &str = "/tmp/dump.txt";
const NUMBEROF_BYTESTO_READ: i32 = 3072;

// https://github.com/SecurityJon/reliableubootdumper/blob/main/reliableubootflashdumper.py
fn print_process_bar(iteration: i32, total: i32) {
    /*
    Call in a loop to create terminal progress bar
    @params:
        iteration   - Required  : current iteration (Int)
        total       - Required  : total iterations (Int)
        prefix      - Optional  : prefix string (Str)
        suffix      - Optional  : suffix string (Str)
        decimals    - Optional  : positive number of decimals in percent complete (Int)
        length      - Optional  : character length of bar (Int)
        fill        - Optional  : bar fill character (Str)
        printEnd    - Optional  : end character (e.g. "\r", "\r\n") (Str)
    */
    // let percent = ("{0:." + decimals + "f}").format(100 * (iteration / total));
    // let filledLength = length * iteration / total
    // let bar = fill * filledLength + '-' * (length - filledLength);
    // println!(f"\r{}|{}| {}% {}", prefix, bar,  percent, suffix, end = printEnd);
    // Print New Line on Complete
    if iteration == total {
        println!("\n");
    }
}

fn fix_output_file() {
    let path = Path::new(FINALFILEPATH);
    if path.exists() {
        let _ = match std::fs::remove_file(FINALFILEPATH) {
            Err(why) =>panic!("couldn't remoce {}", why),
            Ok(ok) => ok,
        };
    }
    let display = path.display();
    let tmp_path = Path::new(TEMPFILEPATH);
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut tmp_file = match File::open(tmp_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    let mut final_file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    // read tmp and write final 
    if tmp_path.exists() {
        let _ = match std::fs::remove_file(FINALFILEPATH) {
            Err(why) =>panic!("couldn't remoce {}", why),
            Ok(ok) => ok,
        };
    }
    
}

fn parse_args() {

}

fn print_header() {
    println!("#######################################################");
    println!("\tReliable Uboot Flash Dumper");
    println!("#######################################################");
    println!("");
    println!("This tool does not handle multiple connections to the serial port well.");
    println!("Please stop anything from using the serial port and press enter when you have done so");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok().expect("Error!"); 
}


fn open_serial_and_read() {
    let mut ser = serial::open(SERIAL_PORT_TO_USE).unwrap();
    interact(&mut ser);
}

fn interact(port: &mut SerialPort) {
    let err = port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600).unwrap();
        Ok(())
    });
    port.set_timeout(Duration::from_millis(1000)).unwrap();

    loop {
        let flash_read_corret = false;
        while !flash_read_corret {
            let flash_dump1 = String::new();
            let flash_dump2 = String::new();
            port.flush();
            let toWrite = ("md.b " + str(flashLocationToReadInHex) + " " + str(numberofBytestoReadInHex) + " \r\n")
            port.write(toWrite);
            std::thread::sleep(Duration::from_millis(100));
            loop {
                let mut repsonse = String::new();
                port.read_to_string(&mut repsonse);

            }
        }
    };
}

fn main() {
    let matches = App::new("Uboot dumper")
        .version("0.1.0")
        .author("xxxx")
        .about("uboot dumper")
        .arg(Arg::with_name("path")
                 .short("p")
                 .long("path")
                 .takes_value(true)
                 .help("Path to the Output file to write to"))
        .arg(Arg::with_name("flash")
                 .short("f")
                 .long("flash")
                 .takes_value(true)
                 .help("The amount of Flash memory to dump, in megabytes, for example, 4"))
        .arg(Arg::with_name("tty")
                 .short("t")
                 .long("tty")
                 .takes_value(true)
                 .help("The serial interface to use, for example, /dev/ttyUSB0"))
        .arg(Arg::with_name("baud")
                 .short("b")
                 .long("baudrate")
                 .takes_value(true)
                 .help("'The baud rate of the serial interface, for example, 115200"))
        .get_matches();
    let serial_port_to_use = matches.value_of("tty").unwrap_or("1234");
    let baudRateToUse = matches.value_of("baud").unwrap_or("1234");       
    let flashSize = matches.value_of("flash").unwrap_or("1234");
    let finalfilepath = matches.value_of("path").unwrap_or("output_data");
    let tempfilepath = format!("{}_temp", finalfilepath);
    println!("{}", tempfilepath);

    print_header();
    let start = Instant::now();
    open_serial_and_read();
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
