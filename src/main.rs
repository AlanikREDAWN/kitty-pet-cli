use std::io::{self};
use std::time::Instant;
use rand::Rng;

fn main() {
    let mut happiness = 0;

    let mut count = 0;
    
    let mut age = 1;

    let mut gonetovet = false;

    let mut gonetobeach = false;

    let mut goneonwalk = false;

    let mut gonetopark = false;

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

    println!("Meet your new kitty: {}", name);

    loop {
        
        let start = Instant::now();

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
        println!("Age: {}", age);
        println!("\n");
        

        if happiness < 0 {
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);
        }

        if count % 5 == 0 {
            println!("Your kitty is now a year older!");
            age += 1;
        }

        if age == 2 && !gonetovet {
            vet(&name);
            gonetovet = true;
        }

        if age == 3 && !gonetobeach {
            beach(&name);
            gonetobeach = true;
        }

        if age == 4 && !goneonwalk {
            walk(&name);
            goneonwalk = true;
        }

        if age == 5 && !gonetopark {
            park(&name);
            gonetopark = true;
        }

        count += 1;
    }

}

fn thursty(happiness: &mut i32, name: &String) -> i32 {
    loop {
        let mut action = String::new();

        let thursty = format!(r#"
{} is thursty! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", thursty);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} drinks water.
{}'s happiness went up by 3", name.trim(), name.trim());

            println!("{}", water);
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", food);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", play);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", snuggle);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", sad);
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

        let hungry = format!(r#"
{} is hungry! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", hungry);

        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", water);
            *happiness -= 2;

            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} eats.
{}'s happiness went up by 3", name.trim(), name.trim());

            println!("{}", food);
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", play);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", snuggle);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", sad);
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

        let playful = format!(r#"
{} wants to play! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", playful);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", water);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", food);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} plays.
{}'s happiness went up by 3", name.trim(), name.trim());

            println!("{}", play);
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} doesn't want to snuggle.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", snuggle);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", sad);
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

        let snuggles = format!(r#"
{} wants to snuggle! What do you do?
        
        a: give {} water
        b: give {} food
        c: play with {}
        d: snuggle with {}
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", snuggles);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let water = format!("
{} doesn't want water.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", water);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "b" {
            let food = format!("
{} doesn't want food.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", food);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "c" {
            let play = format!("
{} doesn't want to play.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", play);
            *happiness -= 2;
            break;
        } else if action.trim().to_lowercase() == "d" {
            let snuggle = format!("
{} snuggles.
{}'s happiness went up by 3", name.trim(), name.trim());

            println!("{}", snuggle);
            *happiness += 3;
            break;
        } else if action.trim().to_lowercase() == "e" {
            let sad = format!("
{} is sad.
{}'s happiness went down by 2", name.trim(), name.trim());

            println!("{}", sad);
            *happiness -= 2;
            break;
        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
    *happiness
}

fn vet(name: &String) {
    loop {
        let mut action = String::new();

        let vet = format!(r#"
{} needs to go to the vet! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", vet);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let tovet = format!("
{} goes to the vet.
{} is perfectly healthy.", name.trim(), name.trim());

            println!("{}", tovet);

            break;
        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s health.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn beach(name: &String) {
    loop {
        let mut action = String::new();

        let beach = format!(r#"
{} needs to go to the beach! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", beach);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "d" {
            let tobeach = format!("
{} goes to the beach.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", tobeach);

            break;

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn walk(name: &String) {
    loop {
        let mut action = String::new();

        let walk = format!(r#"
{} needs to go on a walk! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", walk);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "c" {
            let onwalk = format!("
{} goes on a walk.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", onwalk);

            break;
        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}

fn park(name: &String) {
    loop {
        let mut action = String::new();

        let park = format!(r#"
{} needs to go to the park! What do you do?
        
        a: take {} to the vet
        b: take {} to the park
        c: take {} on a walk
        d: take {} to the beach
        e: ignore {}"#, name.trim(), name.trim(), name.trim(), name.trim(), name.trim(), name.trim());

        println!("{}", park);
        io::Write::flush(&mut io::stdout()).expect("flush failed!");

        match io::stdin().read_line(&mut action) {
            Ok(_) => (),
            Err(err) => println!("Could not parse input: {}", err)
        }

        if action.trim().to_lowercase() == "a" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "b" {
            let topark = format!("
{} goes to the park.
{} is perfectly happy.", name.trim(), name.trim());

            println!("{}", topark);

            break;

        } else if action.trim().to_lowercase() == "c" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);
        } else if action.trim().to_lowercase() == "d" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else if action.trim().to_lowercase() == "e" {
            let ignore = format!("You ignored your {}'s need.", name.trim());

            println!("{}", ignore);
            
            println!("Your kitty ran away! Game over!");
            std::process::exit(0);

        } else {
            println!("\n");
            println!("Invalid action");
        }
    }
}