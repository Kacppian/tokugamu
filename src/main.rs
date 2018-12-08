// #[macro_use]
// extern crate clap;
// use clap::App;
use std::env;

fn main() {
    println!("
        ##      ##     #     # ####### #        #####  ####### #     # #######
        #  #  # #  #  # #  #  # #       #       #     # #     # ##   ## #
            ##      ##  #  #  # #       #       #       #     # # # # # #
                        #  #  # #####   #       #       #     # #  #  # #####
                        #  #  # #       #       #       #     # #     # #
                        #  #  # #       #       #     # #     # #     # #
                         ## ##  ####### #######  #####  ####### #     # #######

        #     # ####### ######  #       #######    #     # #     # #     #    #    #     #  ##      ##
        ##    # #     # #     # #       #          #     # #     # ##   ##   # #   ##    # #  #  # #  #  #
        # #   # #     # #     # #       #          #     # #     # # # # #  #   #  # #   #     ##      ##
        #  #  # #     # ######  #       #####      ####### #     # #  #  # #     # #  #  #
        #   # # #     # #     # #       #          #     # #     # #     # ####### #   # #
        #    ## #     # #     # #       #          #     # #     # #     # #     # #    ##
        #     # ####### ######  ####### #######    #     #  #####  #     # #     # #     #
    ");
    println!("# This game is simple.");
    println!("* An interesting random context will be generated. You'll be assigned characters
            that you'll have to live up to over the course of the conversation");

    let args: Vec<String> = env::args().collect();
    let query = &args[1].to_string();
    println!("{:?}", query);

    match query.trim() {
        "-h" => println!("Yay"),
        "-qc" => println!("Yay"),
        "-q" => println!("quit convo"),
        "-n" => println!("New"),
        _ => println!("Sad")
    }
}
