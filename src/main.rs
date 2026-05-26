use std::io::Write;

#[cfg(feature = "blasphemy")]
use rand::distr::Distribution;
#[cfg(feature = "blasphemy")]
use std::io::Read;

use crate::{config::Config, enneagram::Enneagram, enneatype::Enneatype};

moddef::moddef!(
    mod {
        artwork for cfg(feature = "artwork"),
        config,
        wireframe,
        enneatype,
        line,
        enneagram,
        domain,
        triad,
        personality,
        pivot,
        path
    }
);

fn main()
{
    run(std::env::args())
}

fn run(args: impl IntoIterator<Item: Into<String>>)
{
    let mut args = args.into_iter()
        .map(Into::into)
        .peekable();

    let _executeable = args.next()
        .unwrap_or_else(|| "enneagram".to_string());

    #[cfg(feature = "pivot")]
    let mut enable_pivot = true;
    #[cfg(feature = "artwork")]
    let mut enable_artwork = false;
    let mut enneagram = Enneagram {
        edges: Vec::new(),
        config: Config::default()
    };

    let mut configs = vec![];

    loop
    {
        let argument = match args.next()
        {
            Some(number) => number,
            None => {
                match configs.len()
                {
                    0 => (),
                    1 => [(_, enneagram.config)] = configs.try_into()
                        .expect("Config should now be unambiguous."),
                    2.. => {
                        let options = configs.into_iter()
                            .map(|config| (config.0, move || config.1.clone()))
                            .collect::<Vec<(String, _)>>();

                        enneagram.config = crate::select(
                            Clause::Answer("please select one config"),
                            &options.iter()
                                .map(|config| (config.0.as_str(), &config.1 as &dyn Fn() -> _))
                                .collect::<Vec<_>>()
                        );
                    }
                }

                #[cfg(feature = "artwork")]
                if enable_artwork
                {
                    let mut terminal = ratatui::init();

                    use crate::artwork::Artwork;

                    Artwork {
                        enneagram
                    }.draw(&mut terminal);

                    return
                }
                if enneagram.edges.is_empty()
                {
                    let domain = domain::select(&enneagram.config);
                    let mut edge = domain.edge();

                    let edge_info = core::fmt::from_fn(|f| edge.info(f, &enneagram.config));
                    println!("\n{edge_info}");

                    #[cfg(feature = "pivot")]
                    if enable_pivot
                    {
                        loop
                        {
                            println!();
                            let pivot = edge.pivot();
                            let origin = core::mem::replace(&mut edge, pivot.select(&enneagram.config));
                            if edge == origin
                            {
                                break
                            }

                            let edge_info = core::fmt::from_fn(|f| edge.info(f, &enneagram.config));
                            println!("\n{edge_info}");
                        }
                    }

                    return
                }
                else
                {
                    let mut sep = "";
                    for edges in enneagram.edges
                    {
                        let edge_info = core::fmt::from_fn(|f| Enneatype::common_info(&edges, f, &enneagram.config));
                        println!("{sep}{edge_info}");
                        sep = "\n"
                    }
                    return
                }
            }
        };

        enum Flag
        {
            #[cfg(feature = "pivot")]
            Pivot,
            #[cfg(feature = "artwork")]
            Artwork,
            Config
        }

        let mut take_flag = |flag, invert| {
            match flag
            {
                #[cfg(feature = "pivot")]
                Flag::Pivot => match (enable_pivot, invert)
                {
                    (true, true) => enable_pivot = false,
                    (true, false) => panic!("Pivot is already enabled"),
                    (false, true) => panic!("Pivot is already disabled"),
                    (false, false) => enable_pivot = true
                },
                #[cfg(feature = "artwork")]
                Flag::Artwork => match (enable_artwork, invert)
                {
                    (true, true) => enable_artwork = false,
                    (true, false) => panic!("Artwork is already enabled"),
                    (false, true) => panic!("Artwork is already disabled"),
                    (false, false) => enable_artwork = true
                },
                Flag::Config => if invert
                {
                    configs.clear()
                }
                else if let Some(config_path) = args.next()
                {
                    while let Some(next_arg) = args.peek() && next_arg == ":"
                    {
                        let _ = args.next() // Ignore ':'-operator
                            .expect("Wasn't there supposed to be a ':'-operator there? Confused.");
                        let config_fallback = args.next()
                            .expect(&format!(
                                "Expected argument: additional fallback config-file (yaml, see {}), due to preceding ':'-operator.",
                                Config::default_config_path().to_string_lossy()
                            ));
                        Config::add_fallback(Config::read_config(&config_fallback));
                    }
                    let config = Config::read_config(&config_path);
                    configs.push((config_path, config))
                }
                else
                {
                    panic!("Expected argument: config-file (yaml, see {}).", Config::default_config_path().to_string_lossy())
                }
            }
        };
        let mut invert = false;
        if argument.starts_with("--")
        {
            let mut flag_str = &argument["--".len()..];
            while flag_str.starts_with("!")
            {
                flag_str = &flag_str["!".len()..];
                invert = !invert
            }
            if flag_str.is_empty()
            {
                panic!("Invalid argument: Expected flag")
            }
            let flag = match flag_str
            {
                #[cfg(feature = "pivot")]
                "pivot" => Flag::Pivot,
                #[cfg(feature = "artwork")]
                "artwork" => Flag::Artwork,
                "config" => Flag::Config,
                _ => panic!("Invalid argument: Unrecognized flag '{flag_str}'")
            };
            take_flag(
                flag,
                std::mem::replace(&mut invert, false)
            );
            continue
        }
        else if argument.starts_with("-")
        {
            for flag_char in argument["-".len()..].chars()
            {
                let flag = match flag_char
                {
                    '!' => {
                        invert = !invert;
                        continue
                    }
                    #[cfg(feature = "pivot")]
                    'p' => Flag::Pivot,
                    #[cfg(feature = "artwork")]
                    'a' => Flag::Artwork,
                    'c' => Flag::Config,
                    _ => panic!("Invalid argument: Unrecognized single-character flag '{flag_char}'")
                };
                take_flag(
                    flag,
                    std::mem::replace(&mut invert, false)
                );
            }
            if invert
            {
                panic!("Invalid argument: Expected flag")
            }
            continue
        }
        else if let Ok(mut number) = argument.parse::<u128>().map(Some)
        {
            enneagram.edges.push(
                core::iter::repeat(())
                    .map_while(|()| {
                        let n = number.take()?;
                        let digit = (n % 10) as u8;
                        if n >= 10
                        {
                            number = Some(n/10)
                        }
                        Some(digit)
                    })
                    .map(|digit| Enneatype::new(digit))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect()
            );
            continue
        }
        panic!("Invalid arguments.")
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
    // saves cursor position (ANSI-escape)
    println!("(pick one)");
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
            println!("~ If you want to disable this feature, you can compile this computer program without the `blasphemy` feature.");
            println!("~ Your fate is now in the hands of forces beyond.");
            print!("\n[ Press enter to continue... ]");
            std::io::stdout()
                .flush()
                .expect("Failed to flush stdout");
            let _ = std::io::stdin()
                .read(&mut [0])
                .expect("Failed to wait for any-key");
            // Please forgive me if this is blasphemy...
            let mut divine_intellect = rand::rng(); // We can only hope
            let god_dice = rand::distr::Uniform::try_from(0..options.len() as u8) // This is why gambling is a sin, because you are forcing God's hand to do evil, or you are letting the devil roll the dice
                .expect("Did you not present a nonzero amount of choices to begin with? Regardless, not even God can roll his dice when the range of possibilities is unsound. Make up your mind.");
            let choice_number = god_dice.sample(&mut divine_intellect);
            options.get(choice_number as usize)
                .expect("Due to unforseen events, it seems the number was out of range.")
        }
        #[cfg(not(feature = "blasphemy"))]
        {
            panic!("You are running the blasphemy-free version of the enneagram software. You must make the decision yourself. Machine elves are not enabled due to the risk of blasphemy.")
        }
    }
    else
    {
        assert_eq!(choice_str.len(), 1, "There must have been a mistake. More bytes were read than expected. We only expect one byte to be read here.");
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
        Clause::Continuation(_) => println!("{expression}\x1b[s")
    }
    result()
}

#[cfg(test)]
mod test
{
    use std::path::Path;

    #[cfg(feature = "artwork")]
    #[test]
    #[ignore] // It fucks up the keyboard input, because i'm not fluent in ratatui
    fn test_color_override()
    {
        const YAML: &str = "/tmp/test_color_override_enneagram.yaml";

        std::fs::write(Path::new(YAML), "color:\n  glare: FF00FF\n  sun: FFFF00").unwrap();

        crate::run(["enneagram", "-ac", YAML])
    }

    #[test]
    fn test_christ_enneagram()
    {
        const YAML: &str = "/tmp/chist_enneagram.yaml";

        std::fs::write(Path::new(YAML), include_str!("../presets/christ.yaml")).unwrap();

        crate::run(["enneagram", "-c", YAML, "1"])
    }
}