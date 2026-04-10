#![feature(iter_next_chunk)]

use std::io::Read;

#[cfg(feature = "blasphemy")]
use rand::distr::Distribution;

use crate::{domain::{Behaviour, Domain, ExternalDissonance, InternalConflict}, triad::{Fault, Frame, Triad}};

moddef::moddef!(
    mod {
        edge,
        domain,
        triad
    }
);

fn main()
{
    let domain = domain::select();

    let answer = core::fmt::from_fn(|f| domain.answer(f));

    println!("A: {answer}")
}

fn select<T>(
    question: Option<&str>,
    options: &[(&str, &dyn Fn() -> T)]
) -> T
{
    assert!(options.len() > 0, "No options have been provided. Why ask a question when you provide only the illusion of choice. That's not allowed.");
    let numer_of_options = u8::try_from(options.len())
        .expect("Amount of options cannot exceed 255, due to technical limitations. Because we store the keys of each choice as a byte.");
    if let Some(question) = question
    {
        println!("Q: {question}");
    }
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
        #[cfg(feature = "blasphemy")]
        {
            println!("~ You don't seem to have entered anything at all! Due to your indecisiveness, a choice will be made for you... ~");
            println!("~ Your computer's random number generator will be used to generate a random number. What comes from it, i cannot guarantee. ~");
            println!("~ Whether the elfs that reside in your machine are working for a righteous master, i don't know. It is out of my control. ~");
            println!("~ Your fate is now in the hands of God. ~");
            // Please forgive me if this is blasphemy...
            let mut divine_intellect = rand::rng(); // We can only hope
            let god_dice = rand::distr::Uniform::try_from(1..=options.len() as u8) // This is why gambling is a sin, because you are forcing God's hand to do evil
                .expect("Did you not present a nonzero amount of choices to begin with? Regardless, God cannot roll his dice when the range of possibilities is unsound. Make up your mind.");
            let choice_number = god_dice.sample(&mut divine_intellect);
            options.get(choice_number as usize)
                .expect("God's choice is infallible, but the computer programmer's hands are not. Due to unforseen events, it seems the number was out of range.")
        }
        #[cfg(not(feature = "blasphemy"))]
        {
            panic!("You are running the blasphemy-free version of the enneagram software. You must make the decision yourself. Machine elves are not enabled due to the risk of blasphemy.")
        }
    }
    else
    {
        assert_eq!(number_of_bytes_read, 1, "There must have been a mistake. More bytes were read than expected. We only expect one byte to be read here.");
        let choice_str = str::from_utf8(core::slice::from_ref(&choice_byte))
            .expect("What you wrote was not valid UTF8. Please write a number corresponding to one of the choices presented to you.");
        let choice_number = choice_str.parse::<u8>()
            .expect("What you wrote could not be parsed. Please write a number corresponding to one of the choices presented to you.");
        assert!(choice_number <= numer_of_options, "You tried to select an option that doesn't exist. Your number is out of range. Please write a number corresponding to one of the choices presented to you.");
        let choice_index = choice_number.checked_sub(1)
            .expect("Your number is out of range. Please write a number corresponding to one of the choices presented to you. It cannot be 0.");
        options.get(choice_index as usize)
            .expect("Your number is out of range. Please write a number corresponding to one of the choices presented to you.")
    };
    println!("\x1b 8\x1b[0J"); // restores cursor position, then erases following text (ANSI-escape)
    let (answer, result) = choice;
    if let Some(_) = question
    {
        println!("A: {answer}\n");
    }
    else
    {
        println!("Q: {answer}");
    }
    result()
}


#[cfg(test)]
mod test
{
    #[test]
    fn it_works()
    {
        
    }
}