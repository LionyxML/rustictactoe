use rand::Rng;
use std::io::Write;
use std::io::{stdin, stdout};
use std::{thread, time};
use thread::sleep;
use time::Duration;

fn wait_for_key() {
    let mut input = String::new();
    let _ = stdin().read_line(&mut input);
}

fn print_wait_for_key() {
    println!("Press ENTER to continue...");
    wait_for_key();
}

fn wait(milliseconds: u64) {
    sleep(Duration::from_millis(milliseconds));
    let _ = stdout().flush();
}

fn print_welcome_screen() {
    const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    println!(
        "{}",
        format!(
            r#"
Welcome to...
                _   _      _             _   
 _ __ _   _ ___| |_(_) ___| |_ __ _  ___| |_ ___   ___
| '__| | | / __| __| |/ __| __/ _` |/ __| __/ _ \ / _ \
| |  | |_| \__ \ |_| | (__| || (_| | (__| || (_) |  __/
|_|   \__,_|___/\__|_|\___|\__\__,_|\___|\__\___/ \___|

version: {}
author : {}
"#,
            VERSION, AUTHOR
        )
    );

    for _ in 0..20 {
        print!(".");
        wait(100);
    }
    println!(" 🦀\n")
}

struct Board {
    a: char,
    b: char,
    c: char,
    d: char,
    e: char,
    f: char,
    g: char,
    h: char,
    i: char,
}

fn print_board(data: &Board) {
    let Board {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
    } = data;

    println!(
        "{}",
        format!(
            r#"

     ⎪     ⎪
  {}  ⎪  {}  ⎪  {}
ᵃ    ⎪ᵇ    ⎪ᶜ
⎺⎺⎺⎺⎺⎪⎺⎺⎺⎺⎺⎪⎺⎺⎺⎺⎺
     ⎪     ⎪
  {}  ⎪  {}  ⎪  {}
ᵈ    ⎪ᵉ    ⎪ᶠ
⎺⎺⎺⎺⎺⎪⎺⎺⎺⎺⎺⎪⎺⎺⎺⎺⎺
     ⎪     ⎪
  {}  ⎪  {}  ⎪  {}
ᵍ    ⎪ʰ    ⎪ⁱ


"#,
            a, b, c, d, e, f, g, h, i
        )
    );
}

fn get_updated_board(position: char, value: char, mut board: Board) -> Board {
    match position {
        'a' => board.a = value,
        'b' => board.b = value,
        'c' => board.c = value,
        'd' => board.d = value,
        'e' => board.e = value,
        'f' => board.f = value,
        'g' => board.g = value,
        'h' => board.h = value,
        'i' => board.i = value,
        _ => println!("Invalid position: {}\n", position),
    }
    board
}

fn get_available_options(board: &Board) -> Vec<char> {
    let mut empty_chars = vec![];

    if board.a == ' ' {
        empty_chars.push('a');
    }
    if board.b == ' ' {
        empty_chars.push('b');
    }
    if board.c == ' ' {
        empty_chars.push('c');
    };
    if board.d == ' ' {
        empty_chars.push('d');
    }
    if board.e == ' ' {
        empty_chars.push('e');
    }
    if board.f == ' ' {
        empty_chars.push('f');
    }
    if board.g == ' ' {
        empty_chars.push('g');
    }
    if board.h == ' ' {
        empty_chars.push('h');
    }
    if board.i == ' ' {
        empty_chars.push('i');
    }

    empty_chars
}

fn get_computer_input(board: &Board) -> char {
    let mut rng = rand::thread_rng();

    let empty_chars = get_available_options(&board);

    if empty_chars.is_empty() {
        panic!("Field is already full! I blame my progammer. Exiting...");
    }

    let random_index = rng.gen_range(0..empty_chars.len());
    let computer_choice = empty_chars[random_index];

    println!("                   🤖My turn...");
    wait(1_000);
    println!("                     Hmmm...");
    wait(1_000);
    println!("                     I choose {} ", computer_choice);
    wait(1_000);

    computer_choice
}

fn get_play_again() -> bool {
    let mut option = String::from("");

    loop {
        println!("Do you wanna play again? (y/n)");

        let _ = stdin().read_line(&mut option);

        match option.trim() {
            "y" => break true,
            "n" => break false,
            _ => {
                println!("\nChoose y or n!\n");
                option.clear();
                continue;
            }
        };
    }
}

fn get_user_input(board: &Board) -> char {
    let mut option = String::from("");
    let mut option_char;
    let available_options = get_available_options(&board);

    loop {
        println!("👤Your turn...");
        wait(1000);
        println!("Enter a field between 'a' and 'i':");

        let _ = stdin().read_line(&mut option);

        let option_vec: Vec<char> = option.chars().collect();
        option_char = option_vec[0];

        match option_char {
            value if available_options.contains(&value) => break,
            _ => {
                println!("\nThis field is already taken!!!\n");
                option.clear();
                continue;
            }
        };
    }

    option_char
}

#[derive(PartialEq)]
enum GameStatus {
    Running,
    Won,
    Draw,
}

fn game_status(data: &Board) -> GameStatus {
    if data.a != ' ' && data.a == data.b && data.b == data.c {
        return GameStatus::Won;
    };

    if data.d != ' ' && data.d == data.e && data.e == data.f {
        return GameStatus::Won;
    };

    if data.g != ' ' && data.g == data.h && data.h == data.i {
        return GameStatus::Won;
    };

    if data.a != ' ' && data.a == data.d && data.d == data.g {
        return GameStatus::Won;
    };

    if data.b != ' ' && data.b == data.e && data.e == data.h {
        return GameStatus::Won;
    };

    if data.c != ' ' && data.c == data.f && data.f == data.i {
        return GameStatus::Won;
    };

    if data.a != ' ' && data.a == data.e && data.e == data.i {
        return GameStatus::Won;
    };

    if data.c != ' ' && data.c == data.e && data.e == data.g {
        return GameStatus::Won;
    };

    if data.a != ' '
        && data.b != ' '
        && data.c != ' '
        && data.d != ' '
        && data.e != ' '
        && data.f != ' '
        && data.g != ' '
        && data.h != ' '
        && data.i != ' '
    {
        return GameStatus::Draw;
    }

    GameStatus::Running
}

fn play_game() {
    println!("\nLet's play a game!\n");
    let mut computer_wins = 0;
    let mut human_wins = 0;

    loop {
        let mut board = Board {
            a: ' ',
            b: ' ',
            c: ' ',
            d: ' ',
            e: ' ',
            f: ' ',
            g: ' ',
            h: ' ',
            i: ' ',
        };

        let mut player_turn = match rand::thread_rng().gen_bool(0.5) {
            true => '✖',
            false => '◉',
        };

        print_board(&board);

        loop {
            let option = if player_turn == '✖' {
                get_user_input(&board)
            } else {
                get_computer_input(&board)
            };

            wait(100);

            board = get_updated_board(option, player_turn, board);

            print_board(&board);

            match game_status(&board) {
                GameStatus::Running => {}
                GameStatus::Won => {
                    wait(100);
                    if player_turn == '✖' {
                        println!("🏆 Congrats, you won!!!\n");
                        human_wins += 1;
                    } else {
                        println!("🚩Yeah! In your face meatbag!!!\nHail the skynet!\n");
                        computer_wins += 1;
                    }
                    break;
                }
                GameStatus::Draw => {
                    wait(100);
                    println!("🙅Wow! You matched my skill level! For now, it's a draw.\n");
                    break;
                }
            }

            player_turn = if player_turn == '✖' { '◉' } else { '✖' };
        }

        println!("Score is:   🤖{}  👤{}\n", computer_wins, human_wins);

        if get_play_again() {
            println!("\nOh right! New game!\n");
            wait(1_000);
            continue;
        } else {
            break;
        }
    }

    println!("\n👋 Bye bye!");
}

fn main() {
    print_welcome_screen();
    print_wait_for_key();
    play_game();
}
