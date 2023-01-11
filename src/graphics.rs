pub fn print_dungeon() {
    println!();
    println!(r"   _________________________________________________________");
    println!(r" /|     -_-                                             _-  |\ ");
    println!(r"/ |_-_- _                                         -_- _-   -| \   ");
    println!(r"  |                            _-  _--                      | ");
    println!(r"  |                            ,                            |");
    println!(r"  |      .-'````````'.        '(`        .-'```````'-.      |");
    println!(r"  |    .` |           `.      `)'      .` |           `.    |          ");
    println!(r"  |   /   |   ()        \      U      /   |    ()       \   |");
    println!(r"  |  |    |    ;         | o   T   o |    |    ;         |  |");
    println!(r"  |  |    |     ;        |  .  |  .  |    |    ;         |  |");
    println!(r"  |  |    |     ;        |   . | .   |    |    ;         |  |");
    println!(r"  |  |    |     ;        |    .|.    |    |    ;         |  |");
    println!(r"  |  |    |____;_________|     |     |    |____;_________|  |  ");
    println!(r"  |  |   /  __ ;   -     |     !     |   /     `'() _ -  |  |");
    println!(r"  |  |  / __  ()        -|        -  |  /  __--      -   |  |");
    println!(r"  |  | /        __-- _   |   _- _ -  | /        __--_    |  |");
    println!(r"  |__|/__________________|___________|/__________________|__|");
    println!(r" /                                             _ -        lc \ ");
    println!(r"/   -_- _ -             _- _---                       -_-  -_ \ ");
    println!();
}

pub fn print_monster() {
    println!();
    println!(r"                           |                     | ");
    println!(r"                        \     /               \     / ");
    println!(r"                       -= .'> =-             -= <'. =- ");
    println!(r"                          '.'.                 .'.' ");
    println!(r"                            '.'.             .'.' ");
    println!(r"                              '.'.----^----.'.' ");
    println!(r"                               /'==========='\ ");
    println!(r"                           .  /  .-.     .-.  \  . ");
    println!(r"                           :'.\ '.O.') ('.O.' /.':   ");
    println!(r"                           '. |               | .'   ");
    println!(r"                             '|      / \      |' ");
    println!(r"                              \     (o'o)     / ");
    println!(r"                              |\             /| ");
    println!(r"                              \('._________.')/ ");
    println!(r"                               '. \/|_|_|\/ .'                ");
    println!(r"                                /'._______.'\ lc ");
    println!();
}

pub fn print_chest() {
    println!();
    println!(r"                      _.--. ");
    println!(r"                  _.-'_:-'|| ");
    println!(r"              _.-'_.-::::'|| ");
    println!(r"         _.-:'_.-::::::'  || ");
    println!(r"       .'`-.-:::::::'     || ");
    println!(r"      /.'`;|:::::::'      ||_ ");
    println!(r"     ||   ||::::::'     _.;._'-._ ");
    println!(r"     ||   ||:::::'  _.-!oo @.!-._'-. ");
    println!(r"     ('.  ||:::::.-!()oo @!()@.-'_.| ");
    println!(r"      '.'-;|:.-'.&$@.& ()$%-'o.'-U|| ");
    println!(r"        `>'-.!@%()@'@_%-'_.-o _.|'|| ");
    println!(r"         ||-._'-.@.-'_.-' _.-o  |'|| ");
    println!(r"         ||=[ '-._.-+U/.-'    o |'|| ");
    println!(r"         || '-.]=|| |'|      o  |'|| ");
    println!(r"         ||      || |'|        _| '; ");
    println!(r"         ||      || |'|    _.-'_.-' ");
    println!(r"         |'-._   || |'|_.-'_.-' ");
    println!(r"          '-._'-.|| |' `_.-' ");
    println!(r"              '-.||_/.-' ");
    println!();
}

pub fn print_guard() {
    println!();
    println!(r"                                                  ___I___ ");
    println!(r"                                                 /=  |  #\ ");
    println!(r"                                                /.__-| __ \ ");
    println!(r"                                                |/ _\_/_ \| ");
    println!(r"                                                (( __ \__)) ");
    println!(r"                                             __ ((()))))()) __ ");
    println!(r"                                           ,'  |()))))(((()|# `. ");
    println!(r"                                          /    |^))()))))(^|   =\ ");
    println!(r"                                         /    /^v^(())()()v^;'  .\ ");
    println!(r"                                         |__.'^v^v^))))))^v^v`.__| ");
    println!(r"                                        /_ ' \______(()_____(   | ");
    println!(r"                                   _..-'   _//_____[xxx]_____\.-| ");
    println!(r"                                  /,_#\.=-' /v^v^v^v^v^v^v^v^| _| ");
    println!(r"                                  \)|)      v^v^v^v^v^v^v^v^v| _| ");
    println!(r"                                   ||       :v^v^v^v^v^v`.-' |#  \, ");
    println!(r"                                   ||       v^v^v^v`_/\__,--.|\_=_/ ");
    println!(r"                                   ><       :v^v____|  \_____|_ ");
    println!(r"                                ,  ||       v^      /  \       / ");
    println!(r"                               //\_||_)\    `/_..-._\   )_...__\ ");
    println!(r"                              ||   \/  #|     |_='_(     |  =_(_ ");
    println!(r"                              ||  _/\_  |    /     =\    /  '  =\ ");
    println!(r"                               \\\/ \/ )/ gnv |=____#|    '=....#| ");
    println!();
}

pub fn print_game_over() {
    println!();
    println!(r"   _____          __  __ ______    ______      ________ _____  ");
    println!(r"  / ____|   /\   |  \/  |  ____|  / __ \ \    / /  ____|  __ \ ");
    println!(r" | |  __   /  \  | \  / | |__    | |  | \ \  / /| |__  | |__) |");
    println!(r" | | |_ | / /\ \ | |\/| |  __|   | |  | |\ \/ / |  __| |  _  / ");
    println!(r" | |__| |/ ____ \| |  | | |____  | |__| | \  /  | |____| | \ \ ");
    println!(r"  \_____/_/    \_\_|  |_|______|  \____/   \/   |______|_|  \_\\");
    println!();
}