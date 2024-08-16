use bevy::prelude::*;
use std::io;
use std::io::*;
use std::fs::{File, Permissions, OpenOptions};

fn main() {
    App::new()
        .add_systems(Startup, write)
        .run();
}
fn readfile(b: bool) -> bool{
    if b == false{
        let ile_result = File::open("src/.b");
        let mut ile = match ile_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("src/.b") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}. Come on man. It's a skill issue."),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}. Maybe try sudo next time.");
                }
            },
        };
        
        let mut c = String::new();
        ile.read_to_string(&mut c);
        if c == "" {
            ile.write_all(b"0\n");
            return false;
        }
        if c == "0\n"{
            return false;
        }
        if c == "1\n"{
            return true;
        }
        else {
            return false;
        }
    }
    if b == true {
        let mut ile = OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .open("src/.b")
                            .unwrap();
        let meta = ile.metadata().unwrap();
        ile.write_all(b"1\n");
        return true;
    }
    else {
        return true
    }
}

fn write() {
    let mut m = 0;
    println!("What is my name? Take a gander. Only 8 characters.");
    let ban = readfile(false);
    let  mut story = 0;
    let mut intime = true;
    loop {
        if ban == true{
            println!("Oh wait. You've been banned.");
            break;
        }
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read the line. Maybe try something valid");
        m = m + 1;
        
        let name = guess.clone();
        let g = respond(guess, m, intime);
        println!("{:?}", g);
        if name == "Yourmom\n"{
            println!("You're banned.");
            let ban = readfile(true);
            break;
        }
        if name == "Savior\n"{
            if m == 33 && story == 0{
                story += 1;
            }
            if m == 34 && story == 1 {
                story += 1; 
            }
            if m == 35 && story == 2 {
                story += 1;
            }
            if m == 36 && story == 3 {
                story += 1;
            }
            if m == 37 && story == 4 {
                story += 1;
            }
            if m == 38 && story == 5 {
                story += 1;
            }
            else {
                intime = false; 
            }
        }
        if name == "John\n"{
            println!("It took you {:?} tries", m);
            break;
        }
        if name == "Youremom\n"{
            println!("You're banned.");
            let ban = readfile(true);
            break;
        }
    }

}

fn respond(response: String, tries: i32, intime: bool) -> &'static str{
    if response == "John\n"{
        if tries == 1 {
            return "You are cheating aren't you? No one gets this first try.";
        }
        else {
            return "Yep it's John! But next time the game will be harder.";

        }
    }
    if response == "Farside\n" { 
        return "Great comic! One of my favorites deals with Dear John.";
    }
    if response == "Dearjohn\n" {
        return "I haven't got one of these yet! Granted I may because my name is in it."
    }
    if response == "Gander\n" {
        return "Yes, you take one of these. No stealing though";
    }
    if response == "Stealing\n" {
        return "Don't.";
    }
    if response == "Dont\n" {
        return "It's a conjuntion of do and not. Yoda says this non conjucntioned version in his qoute 'do or do not there is no try'";
    }
    if response == "Yoda\n"{
        return "Wrong path you have gone down hmm. Different path you must take."
    }
    if response == "Thor\n" {
        return "GREAT PROGRAMMER. Love this man. He is a great movitvator."
    }
    if response == "Failboat\n" {
        return "The idea is simple."
    }
    if response == "Jaymoji\n"{
        return "That guy who yells BOMB! a lot."
    }
    if response == "Yourmom\n"{
        return "I am banning you."
    }
    if response == "Dear\n" {
        return "Why do you think my name is Dear?";
    }
    if response == "Wallace\n"{
        return "The Brave? Great comic. Farside is great."
    }
    if response == "Calvin\n"{
        return "AND HOBBBES!"
    }
    if response == "Hobbes\n"{
        return "Calvin and Hobbes is great. Reminds me of Wallace."
    }
    if response == "Again\n"{
        return "Very clever... Suppose you get yourself a hint. Calvin."
    }
    if response == "Brave\n"{
        return "I am not brave. Sorry"
    }
    if response == "Sorry\n"{
        return "You will be."
    }
    if response == "Thetruth\n" {
        if tries >= 10 {
            return "You seek what may be most deadly. I send you on a quest."
        }
        else {
            return "Don't try to find it, for your sake."
        }
    }
    if response == "Quest\n" {
        if tries >= 10{
            return "The pen is greater than the sword. Many have tried to find the truth but have not."
        }
        else{
            return "Yes you're on one to find the name."
        }
    }
    if response == "Pen\n" {
        if tries >= 10{
            return "The truth is the killer of gods. It is powerful enough to destroy planets"
        }
        else {
            return "An elegant writing utensil."
        }
    }
    if response == "Planets\n"{
        if tries >= 10 {
            return "And yet you still wish to seek it. I cannot tell you must seek."
        }
        else {
            return "YOU LIVE ON ONE!! YAY!!!"
        }
    }
    if response == "Seek\n" {
        if tries <= 30 {
            return "You will not find it at this time. I refuse you."
        }
        if tries == 31 {
            return "The Truth of the universe. If you wish to find it you must go upon a steady path. Make no error."

        }
        else {
            return "Are you still looking for my name?"
        }
    }
    if response == "Universe\n" {
        if tries == 32 { 
            return "The truth is complex. Depending on which path you take the truth is different."
        }
        else {
            return "Good grief you think the answer is the universe? Try 'again'"
        }
    }
    if response == "Complex\n" {
        if tries == 33 {
            return "Like all things are. From mistakes of the past we must try again"
        }
    }
    if response == "Path\n" {
        if tries == 32 {
            return "The truth is strange. It is both the savior and death of life."
        }
        else {
            return "You need to look for one. Hop to it."
        }
    }
    if response == "Savior\n"{
        if intime == true{
            if tries == 33 {
                return "I'm sorry you will not find a savor in the truth."
            }
            if tries == 34 {
                return "STOP TRYING TO FIND IT!"
            }
            if tries == 35 {
                return "Fine. I'll tell you a story."
            }
            if tries == 36 {
                return "Once there was war in the lands. Before our time. Then Ubuntu came."
            }
            if tries == 37 {
                return "The gods of this world were angry with the one who created it."
            }
            if tries == 38 {
                return "Ubuntu though brought peace to the world. The gods have ignored us since."
            }
            if tries == 39 {
                return "I have no more story to tell. I wish you on your way."
            }
            else {
                return "No I'm no savor."
            }
        }
        else {
            return "No I'm no savor. "
        }
    }
    if response == "Way\n"{
        return "When is there is a will there is one of these."
    }
    if response == "Will"{
        return "There are many wills. Like the will to live. The will to eat. The will to code nonstop."
    }
    if response == "Live"{
        return "BREAKING NEWS! WE GO LIVE TO JOHN WHO IS ON THE SCENE!"
    }
    if response == "Eat"{
        return "Great now you made me hungry. Come on man."
    }
    if response == "Nonstop"{
        return "Would put a song here. But I don't want to have copyright issues"
    }
    if response == "Youremom\n"{
        return "Good job spelling. Your're still banned."
    }
    else {
        return "Sorry that name isn't in my database... Try again"
    }

}