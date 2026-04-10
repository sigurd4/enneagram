#![feature(iter_next_chunk)]

use std::io::Read;

use rand::distr::Distribution;

use crate::{domain::{Domain, InternalConflict}, triad::{Fault, Frame}};

moddef::moddef!(
    mod {
        edge,
        domain,
        triad
    }
);

fn select<T>(
    question: &str,
    options: &[(&str, &dyn Fn() -> T)]
) -> T
{
    assert!(options.len() > 0, "No options have been provided. Why ask a question when you provide only the illusion of choice. That's not allowed.");
    let numer_of_options = u8::try_from(options.len())
        .expect("Amount of options cannot exceed 255, due to technical limitations. Because we store the keys of each choice as a byte.");
    println!("Q: {question}");
    println!("\x1b(pick one)"); // saves cursor position (ANSI-escape)
    for (n, (choice, _)) in options.iter()
        .enumerate()
    {
        println!("\t{n}. {choice}")
    }
    print!("[sA: ");
    let mut choice_byte = 0;
    let number_of_bytes_read = std::io::stdin()
        .read(core::slice::from_mut(&mut choice_byte))
        .expect("We failed to read the input because of an input-output error from your operating-system.");
    let choice = if number_of_bytes_read == 0
    {
        println!("~ You don't seem to have entered anything at all! Due to your indecisiveness, a choice will be made for you... ~");
        println!("~ Your computer's random number generator will be used to generate a random number. What comes from it, i cannot guarantee. ~");
        println!("~ Whether the elfs that reside in your machine are working for a righteous master, i don't know. It is out of my control. ~");
        println!("~ Your fate is now in the hands of God. ~");
        // Please forgive me if this is blasphemy...
        let mut divine_intellect = rand::rng(); // We can only hope
        let god_dice = rand::distr::Uniform::try_from(1..=options.len() as u8)
            .expect("Did you not present a nonzero amount of choices to begin with? Regardless, God cannot roll his dice when the range of possibilities is unsound. Make up your mind.");
        let choice_number = god_dice.sample(&mut divine_intellect);
        options.get(choice_number as usize)
            .expect("God's choice is infallible, but the computer programmer's hands are not. Due to unforseen events, it seems the number was out of range.")
    }
    else
    {
        assert_eq!(number_of_bytes_read, 1, "There must have been a mistake. More bytes were read than expected. We only expect one byte to be read here.");
        let choice_str = str::from_utf8(core::slice::from_ref(&choice_byte))
            .expect("What you wrote was not valid UTF8. Please write a number corresponding to one of the choices presented to you.");
        let choice_number = choice_str.parse::<u8>()
            .expect("What you wrote could not be parsed. Please write a number corresponding to one of the choices presented to you.");
        options.get(choice_number as usize)
            .expect("You tried to select an option that doesn't exist. Your number is out of range. Please write a number corresponding to one of the choices presented to you.")
    };
    println!("\x1b 8\x1b[0J"); // restores cursor position, then erases following text (ANSI-escape)
    let (answer, result) = choice;
    println!("A: {answer}");
    result()
}

struct Program
{

}


fn main()
{
    let cotriad = InternalConflict {
        thesis: Frame::Gut,
        anti_thesis: Fault::Competent
    };

    let a = std::fmt::from_fn(|f| cotriad.trivial(f));
    println!("{}", a);
}


#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}