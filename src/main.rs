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
    println!("* An interesting random context will be generated. You'll be assigned characters that you'll have to live up to over the course of the conversation");
    println!();

    let args: Vec<String> = env::args().collect();
    let query = &args[1].to_string();

    match query.trim() {
        "-h" => help(),
        "-q" => quit(),
        "-qc" => println!("quit convo"),
        "-n" => println!("New"),
        _ => println!("Sad")
    }
}

fn help() -> () {
    println!("Please raise an issue on our github page - https://github.com/Kacppian/tokugamu");
}

fn quit() -> () {
    println!("Please come back.....");
    std::process::exit(0);
}
