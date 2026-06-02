use clap::{Subcommand, Parser};

#[derive(Parser,Debug)]
#[command(name = "my-cli", version="1.0", about="practice cli")]

struct CLI {
    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long,default_value_t = 1)]
    count: u8,

    #[command(subcommand)]
    command: Option<Commands>,

}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        name : String,
    },

    List {
        #[arg(short,long)]
        verbose: bool,
    },
}

fn main() {
    let cli = CLI::parse();

    match &cli.name {
        Some(n) => {
            for _ in 0..cli.count {
        println!("Hello {n}",)
    }
        }

        None => {
            println!("Nothing Provided")
        }

        
    }

    match &cli.command {
        Some(Commands::Add { name }) => {
             println!("Adding: {}", name)
        },

        Some(Commands::List { verbose }) => {
            if *verbose {
               println!("Listing all items verbosely..."); 
            } else {
                println!("Listing items...");
            }
        }

        None => {
            println!("No subcommand was used.");
        }
    }

    

}