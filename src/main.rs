use std::env;

mod new_game;

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
        "-n" => new_game::start_game(),
        _ => println!("Please select a valid option.")
    }
}

fn help() -> () {
    println!("Please raise an issue on our github page - https://github.com/Kacppian/tokugamu");
}
