#![feature(iter_next_chunk)]

use std::io::{Read, Write};

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

    let enneagram_edge = domain.edge();
    let enneagram_number = enneagram_edge.number();
    let triads = enneagram_edge.triads();

    println!("\nEnneagram {enneagram_number} {enneagram_edge}\n");
    for (m, triad) in triads.into_iter()
        .enumerate()
    {
        let n = m.checked_add(1)
            .and_then(|n| u8::try_from(n).ok())
            .expect("Triad indices are always within the range of 1-9.");
        let numbers = triad.edges()
            .into_iter()
            .map(|edge| edge.number())
            .map(|number| format!("{number}"))
            .collect::<String>();
        println!("{n}: {triad} {numbers}");
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Clause<'a>
{
    Question,
    Answer(&'a str),
    Continuation(&'a str)
}
 
fn select<T>(
    clause: Clause<'_>,
    options: &[(&str, &dyn Fn() -> T)]
) -> T
{
    assert!(options.len() > 0, "No options have been provided. Why ask a question when you provide only the illusion of choice. That's not allowed.");
    let numer_of_options = u8::try_from(options.len())
        .expect("Amount of options cannot exceed 255, due to technical limitations. Because we store the keys of each choice as a byte.");
    match clause
    {
        Clause::Question => print!("\x1b[s"),
        Clause::Answer(question) => println!("Q: {question}\x1b[s"),
        Clause::Continuation(conjunction) => println!("\x1b[u\x1b[0J{conjunction}\x1b[s")
    }
    println!("(pick one)"); // saves cursor position (ANSI-escape)
    for (n, (choice, _)) in options.iter()
        .enumerate()
        .map(|(m, option)| m.checked_add(1)
            .and_then(|n| u8::try_from(n).ok())
            .map(|n: u8| (n, option))
            .expect("Choice-numbers cannot exceed 255, due to technical limitations. Because we store the keys of each choice as a byte.")
        )
    {
        println!("\t{n}. {choice}")
    }
    let mut choice_string = String::new();
    std::io::stdout()
        .flush()
        .expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut choice_string)
        .expect("We failed to read the input because of an input-output error from your operating-system.");
    let choice_str = choice_string.trim();
    let choice = if choice_str.is_empty()
    {
        #[cfg(feature = "blasphemy")]
        {
            use std::io::Write;

            println!("~ You don't seem to have entered anything at all! Due to your indecisiveness, a choice will be made for you...");
            println!("~ Your computer's random number generator will be used to generate a random number. What comes from it, i cannot guarantee.");
            println!("~ Whether the elfs that reside in your machine are working for a righteous master, i don't know. It is out of my control.");
            println!("~ Your fate is now in the hands of God.");
            print!("\n[ Press enter to continue... ]");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout");
            let _ = std::io::stdin()
                .read(&mut [0])
                .expect("Failed to wait for any-key");
            // Please forgive me if this is blasphemy...
            let mut divine_intellect = rand::rng(); // We can only hope
            let god_dice = rand::distr::Uniform::try_from(0..options.len() as u8) // This is why gambling is a sin, because you are forcing God's hand to do evil
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
        assert_eq!(choice_str.len(), 1, "There must have been a mistake. More bytes were read than expected. We only expect one byte to be read here.");
        //let choice_str = str::from_utf8(core::slice::from_ref(&choice_byte))
        //    .expect("What you wrote was not valid UTF8. Please write a number corresponding to one of the choices presented to you.");
        let choice_number = choice_str.parse::<u8>()
            .expect("What you wrote could not be parsed. Please write a number corresponding to one of the choices presented to you.");
        assert!(choice_number <= numer_of_options, "You tried to select an option that doesn't exist. Your number is out of range. Please write a number corresponding to one of the choices presented to you.");
        let choice_index = choice_number.checked_sub(1)
            .expect("Your number is out of range. Please write a number corresponding to one of the choices presented to you. It cannot be 0.");
        options.get(choice_index as usize)
            .expect("Your number is out of range. Please write a number corresponding to one of the choices presented to you.")
    };
    print!("\x1b[u\x1b[0J"); // restores cursor position, then erases following text (ANSI-escape)
    let (expression, result) = choice;
    match clause
    {
        Clause::Question => println!("\nQ: {expression}\x1b[s"),
        Clause::Answer(_) => println!("\nA: {expression}\x1b[s"),
        Clause::Continuation(_) => println!("{expression}\x1b[s"),
    }
    result()
}