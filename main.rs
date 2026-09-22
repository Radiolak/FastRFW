use clap::{Parser, ValueEnum};

// 1. Définition des énumérations pour typer fortement les choix de l'utilisateur

#[derive(ValueEnum, Clone, Debug)]
enum Direction {
    In,
    Out,
}

#[derive(ValueEnum, Clone, Debug)]
enum Action {
    Add,
    Remove,
}

// 2. Définition de la structure des arguments attendus par FastRFW

#[derive(Parser, Debug)]
#[command(
    name = "FastRFW",
    version = "1.0",
    about = "Basic and lightweght cli firewall using Netfilter Queues",
    long_about = None
)]
struct Args {
    /// Direction of the network traffic [IN/OUT]
    #[arg(short = 'd', value_enum)]
    direction: Option<Direction>,

    /// Action to perform [ADD/REMOVE]
    #[arg(short = 'a', value_enum)]
    action: Option<Action>,

    /// Network protocol (e.g., tcp, udp, icmp, all)
    #[arg(short = 'p')]
    protocol: Option<String>,

    /// Priority/Weight of the rule
    #[arg(short = 'c')]
    priority: Option<u32>,

    /// Target IP address or Port number
    #[arg(short = 't')]
    target: Option<String>,

    /// Get current active filters
    #[arg(short = 'i', action = clap::ArgAction::SetTrue)]
    get_filters: bool,
}

fn main() {
    // Analyse (parse) les arguments de la ligne de commande
    let args = Args::parse();

    // Gestion de la commande d'affichage des filtres : fastrfw -i
    if args.get_filters {
        println!("[-] Fetching and displaying active filters...");
        return;
    }

    // Validation minimale pour s'assurer que si l'utilisateur n'utilise pas -i,
    // il fournit au moins les arguments principaux pour configurer une règle.
    if let (Some(dir), Some(act)) = (args.direction, args.action) {
        println!("🔧 Action detected:");
        println!("   Direction : {:?}", dir);
        println!("   Action    : {:?}", act);
        println!("   Protocol  : {:?}", args.protocol.unwrap_or_else(|| "all".to_string()));
        println!("   Priority  : {:?}", args.priority.unwrap_or(0));
        println!("   Target    : {:?}", args.target.unwrap_or_else(|| "any".to_string()));
    } else {
        println!("Error: Missing arguments. Use -h for help.");
    }
