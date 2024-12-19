// use image;
// use artem;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::Rng;

fn main() {
    let mut happiness = 0;

    loop {
        let mut start = String::new();
        println!("Welcome! This program will give you your own virtual kitty over the command line! Would you like to get started?");
        println!("a: yeah!");
        println!("b: help me out");
        println!("c: quit");

        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut start) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if start.trim().to_lowercase() == "a" {
            break;
        } else if start.trim().to_lowercase() == "b" {
            println!("This program will give you your own virtual kitty to take care of. You will be given options to take care of your kitty. You can give your kitty water, food, play with it, or snuggle with it. You can also choose to ignore your kitty (please don't!). Your kitty will have a happiness level that will change based on your actions. If your kitty's happiness level reaches below 0, your kitty will run away. Have fun!");
            println!("\n");
        } else if start.trim().to_lowercase() == "c" {
            std::process::exit(0);
        } else {
            println!("Invalid action");
        }
    }

    let mut name = String::new();
    println!("Name your kitty: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");

    match io::stdin().read_line(&mut name) {
        Ok(_) => (),
        Err(err) => println!("Could not parse input: {}", err)
    }
    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Failed to read line");

    println!("Meet your new kitty: {}", name);

    loop {
        
        let start = Instant::now();
        // println!("Hello, world!");
        // let image = image::open("IMG_4717.jpeg").expect("Failed to open image");
        // let ascii_art = artem::convert(image, &artem::config::ConfigBuilder::new().build());
        // print!("{}", ascii_art);

        println!(r"    /\_____/\");
        println!(r"   /  o   o  \");
        println!(r"  ( ==  ^  == )");
        println!(r"   )         (");
        println!(r"  (           )");
        println!(r" ( (  )   (  ) )");
        println!(r"(__(__)___(__)__)");
        while start.elapsed().as_secs() < 5 {
            
        }


        let to_run = rand::thread_rng().gen_range(1..=4);

        if to_run == 1 {
            thursty(&mut happiness, &name);
        } else if to_run == 2 {
            hungry(&mut happiness, &name);
        } else if to_run == 3 {
            playful(&mut happiness, &name);
        } else {
            snuggles(&mut happiness, &name);
        }

        println!("\n");
        println!("Happiness: {}", happiness);
        println!("\n");
        

        if happiness < 0 {
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);
        }
    }

}

fn thursty(happiness: &mut i32, name: &String) -> i32 {
    loop {
        let mut action = String::new();

        println!("{name} is thursty! What do you do? ");
        println!("a: give {name} water");
        println!("b: give {name} food");
        println!("c: play with {name}");
        println!("d: snuggle with {name}");
        println!("e: ignore {name}");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }
    
        // io::stdin()
        //     .read_line(&mut action)
        //     .expect("Failed to read line");

        if action.trim().to_lowercase() == "a" {
            println!("\n");
            println!("{name} drinks water");
            println!("{name} happiness went up by 3");
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "b" {
            println!("\n");
            println!("{name} doesn't want food");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            println!("\n");
            println!("{name} doesn't want to play");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            println!("\n");
            println!("{name} doesn't want to snuggle");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            println!("\n");
            println!("{name} is sad");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
    *happiness
}

fn hungry(happiness: &mut i32, name: &String) -> i32 {
    loop {
        let mut action = String::new();

        println!("{name} is hungry! What do you do? ");
        println!("a: give {name} water");
        println!("b: give {name} food");
        println!("c: play with {name}");
        println!("d: snuggle with {name}");
        println!("e: ignore {name}");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }
    
        // io::stdin()
        //     .read_line(&mut action)
        //     .expect("Failed to read line");

        if action.trim().to_lowercase() == "a" {
            println!("\n");
            println!("{name} doesn't want water");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;

            break;
        } else if action.trim().to_lowercase() == "b" {
            println!("\n");
            println!("{name} eats");
            println!("{name} happiness went up by 3");
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "c" {
            println!("\n");
            println!("{name} doesn't want to play");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            println!("\n");
            println!("{name} doesn't want to snuggle");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            println!("\n");
            println!("{name} is sad");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
    *happiness
}

fn playful(happiness: &mut i32, name: &String) -> i32 {
    loop {
        let mut action = String::new();

        println!("{name} wants to play! What do you do? ");
        println!("a: give {name} water");
        println!("b: give {name} food");
        println!("c: play with {name}");
        println!("d: snuggle with {name}");
        println!("e: ignore {name}");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }
    
        // io::stdin()
        //     .read_line(&mut action)
        //     .expect("Failed to read line");

        if action.trim().to_lowercase() == "a" {
            println!("\n");
            println!("{name} doesn't want water");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            println!("\n");
            println!("{name} doesn't want food");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            println!("\n");
            println!("{name} plays");
            println!("{name} happiness went up by 3");
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "d" {
            println!("\n");
            println!("{name} doesn't want to snuggle");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            println!("\n");
            println!("{name} is sad");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
    *happiness
}

fn snuggles(happiness: &mut i32, name: &String) -> i32 {
    loop {
        let mut action = String::new();

        println!("{name} wants to snuggle! What do you do? ");
        println!("a: give {name} water");
        println!("b: give {name} food");
        println!("c: play with {name}");
        println!("d: snuggle with {name}");
        println!("e: ignore {name}");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }
    
        // io::stdin()
        //     .read_line(&mut action)
        //     .expect("Failed to read line");

        if action.trim().to_lowercase() == "a" {
            println!("\n");
            println!("{name} doesn't want water");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            println!("\n");
            println!("{name} doesn't want food");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            println!("\n");
            println!("{name} doesn't want to play");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            println!("\n");
            println!("{name} snuggles");
            println!("{name} happiness went up by 3");
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "e" {
            println!("\n");
            println!("{name} is sad");
            println!("{name}'s happiness went down by 2");
            *happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
    *happiness
}