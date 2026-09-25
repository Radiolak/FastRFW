// module de la bibliothèque standard pour utiliser args ici tel que std::env::args
use std::env;


struct ArgsStruct  {
    direction: String, // -d [IN/OUT]
    action: String,    // -a [ADD/REMOVE]
    protocol: String,  // -p [protocol]
    priority: String,  // -c [priority]
    target: String,    // -t [IP / PORT]
}

// déclaration de la fonction argparse, -> sert à déclarer le retour de la fonction
fn argparse() -> ArgsStruct  {

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 6 {
        println!("Erreur : Nombre d'arguments incorrect !");
        println!("Usage   : fastrfw [IN/OUT] [ADD/REMOVE] [protocol] [priority] [IP/PORT]");
        println!("Exemple : fastrfw IN ADD tcp 10 192.168.1.50");
        std::process::exit(1);
    }
    
    ArgsStruct  {
        direction: args[1].clone(),
        action: args[2].clone(),
        protocol: args[3].clone(),
        priority: args[4].clone(),
        target: args[5].clone(),
    }
}

fn main() {
    let arguments = argparse();

    println!("Variables : {} {} {} {} {}", 
        arguments.direction, arguments.action, arguments.protocol, arguments.priority, arguments.target
    );
}
