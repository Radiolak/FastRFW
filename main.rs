// module de la bibliothèque standard pour utiliser args ici tel que std::env::args
use std::env;


struct ArgsStruct  {
    direction: String, // -d [IN/OUT]
    action: String,    // -a [ADD/REMOVE]
    protocol: String,  // -p [protocol]
    target: String,    // -t [IP / PORT]
    priority: String,  // -c [priority]
}

// déclaration de la fonction argparse, -> sert à déclarer le retour de la fonction
fn argparse() -> ArgsStruct  {

    let args: Vec<String> = env::args().collect();
    
    let error_prototype;
        error_prototype = "Usage   : fastrfw [IN/OUT] [ADD/REMOVE] [udp/tcp] [IP or PORT] [priority(1 to 1000)]";
    
     // nombre total d'arguments
    if args.len() != 6 {
        println!("Error: Missing Arguments");
        
        println!("{}", error_prototype);
        std::process::exit(1);
    }
    
    // uniquement IN ou OUT pour la direction
    if args[1] != "IN" && args[1] != "OUT" {
        println!("{}", error_prototype);
        std::process::exit(1);
    }

    // uniquement ADD ou REMOVE pour l'action
    if args[2] != "ADD" && args[2] != "REMOVE" {
        println!("{}", error_prototype);
        std::process::exit(1);
    }

    // uniquement tcp ou udp
    if args[3] != "tcp" && args[3] != "udp" {
        println!("{}", error_prototype);
        std::process::exit(1);
    }

    // cible port ou ip
    let target_str = &args[4];
    let mut target_valide = false;

    // vérification port
    if let Ok(port_num) = target_str.parse::<u32>() {
        if port_num <= 65535 {
            target_valide = true;
        } else {
            println!("{}", error_prototype);
            std::process::exit(1);
        }
    } 
    // vérification ip
    else {
        let segments: Vec<&str> = target_str.split('.').collect();
        if segments.len() == 4 {
            let mut tous_les_blocs_ok = true;
            for segment in segments {
                if let Ok(bloc_num) = segment.parse::<u32>() {
                    if bloc_num > 255 {
                        tous_les_blocs_ok = false;
                    }
                } else {
                    tous_les_blocs_ok = false;
                }
            }
            if tous_les_blocs_ok {
                target_valide = true;
            }
        }
    }

    // si aucun des deux erreure
    if !target_valide {
        println!("{}", error_prototype);
        std::process::exit(1);
    }
    
    // entier de 1 à 1000
    if let Ok(priority_num) = args[5].parse::<u32>() {
        if priority_num < 1 || priority_num > 1000 {
            println!("{}", error_prototype);
            std::process::exit(1);
        }
    } else {
        println!("{}", error_prototype);
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
