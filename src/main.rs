use std::{io::Write, ops::Add};

#[cfg(feature = "blasphemy")]
use rand::distr::Distribution;
#[cfg(feature = "blasphemy")]
use std::io::Read;

use enneagram::{Enneagram, Enneatype, config::{Config, EnneagramConfig, Fallback, Property}, domain::{BodyWithoutOrgans, DesireMachine, Domain, ExternalDissonance, ExternalSynthesis, InternalDissonance, InternalSynthesis}, pivot::Pivot, triad::{Fault, Frame, Means, Need, Triad}};

moddef::moddef!(
    mod {
        artwork for cfg(feature = "artwork")
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
    let mut enneagram = Enneagram::default();

    let mut configs = Vec::<(String, Vec<Config>)>::new();

    loop
    {
        let argument = match args.next()
        {
            Some(number) => number,
            None => {
                match configs.len()
                {
                    0 => (),
                    1 => {
                        let [(_, config)] = configs.try_into()
                            .expect("Config should now be unambiguous.");
                        enneagram.overlay_configs(config);
                    },
                    2.. => {
                        let options = configs.into_iter()
                            .map(|config| (config.0, move || config.1.clone()))
                            .collect::<Vec<(String, _)>>();

                        enneagram.overlay_configs(crate::select::<Vec<Config>>(
                            Clause::Answer("please select one config"),
                            &options.iter()
                                .map(|config| (config.0.as_str(), &config.1 as &dyn Fn() -> _))
                                .collect::<Vec<_>>()
                        ).into_iter().rev());
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
                if enneagram.is_empty()
                {
                    let domain = select_domain(enneagram.config(), enneagram.fallback());
                    let mut edge = domain.edge();

                    let edge_info = core::fmt::from_fn(|f| edge.info(f, enneagram.config(), enneagram.fallback()));
                    println!("\n{edge_info}");

                    #[cfg(feature = "pivot")]
                    if enable_pivot
                    {
                        loop
                        {
                            println!();
                            let pivot = edge.pivot();
                            let origin = core::mem::replace(&mut edge, select_pivot(pivot, enneagram.config(), enneagram.fallback()));
                            if edge == origin
                            {
                                break
                            }

                            let edge_info = core::fmt::from_fn(|f| edge.info(f, enneagram.config(), enneagram.fallback()));
                            println!("\n{edge_info}");
                        }
                    }

                    return
                }
                else
                {
                    let mut sep = "";
                    for edges in enneagram.edges()
                    {
                        let edge_info = core::fmt::from_fn(|f| Enneatype::common_info(edges, f, enneagram.config(), enneagram.fallback()));
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
                    let mut config_grouping = vec![Config::read_config(&config_path)];
                    while let Some(next_arg) = args.peek() && next_arg == ":"
                    {
                        let _ = args.next() // Ignore ':'-operator
                            .expect("Wasn't there supposed to be a ':'-operator there? Confused.");
                        let config_fallback = args.next()
                            .unwrap_or_else(|| panic!(
                                "Expected argument: additional fallback config-file (yaml, see {}), due to preceding ':'-operator.",
                                Config::default_config_path().to_string_lossy()
                            ));
                        config_grouping.push(Config::read_config(&config_fallback));
                    }
                    configs.push((config_path, config_grouping))
                }
                else
                {
                    panic!("Expected argument: config-file (yaml, see {}).", Config::default_config_path().to_string_lossy())
                }
            }
        };
        let mut invert = false;
        if let Some(mut flag_str) = argument.strip_prefix("--")
        {
            while let Some(flag_str_stripped) = flag_str.strip_prefix("!")
            {
                flag_str = flag_str_stripped;
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
        else if let Some(flag_str) = argument.strip_prefix("-")
        {
            for flag_char in flag_str.chars()
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
            enneagram.push_edges(
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
                    .map(Enneatype::new)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
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
    assert!(!options.is_empty(), "No options have been provided. Why ask a question when you provide only the illusion of choice. That's not allowed.");
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

pub fn select_domain(config: &(impl Property<EnneagramConfig> + ?Sized), fallback: &Fallback) -> Box<dyn Domain>
{
    let config = config.property(fallback);

    fn select_triads<T, N>(
        trivial_conjunction: &str,
        trivial: [T; 3],
        nontrivial_conjunction: &str,
        nontrivial: [N; 3],
        config: &EnneagramConfig,
        fallback: &Fallback
    ) -> <T as Add<N>>::Output
    where
        T: Triad + Copy + Add<N, Output: Domain>,
        N: Triad + Copy
    {
        enum Triviality<T, N>
        {
            Trivial(T),
            Nontrivial(N)
        }
        
        let trivial_choices = trivial.each_ref().map(|triad| (triad.config(config, fallback), move || *triad));
        let nontrivial_choices = nontrivial.each_ref().map(|triad| (triad.config(config, fallback), move || *triad));

        let (domain_kind, codomain_kind) = {
            let [(_, lhs), ..] = trivial_choices;
            let [(_, rhs), ..] = nontrivial_choices;
            let domain = lhs() + rhs();
            (domain.kind(config, fallback), domain.reciprocal().kind(config, fallback))
        };

        println!("\x1b[u\x1b[3;90m -> {codomain_kind}\x1b[0m");

        let polymorphic_trivial_choices = trivial_choices.each_ref()
            .map(|(config, generator)| (config.expression.as_ref(), || Triviality::Trivial(generator())));
        let polymorphic_nontrivial_choices = nontrivial_choices.each_ref()
            .map(|(config, generator)| (config.expression.as_ref(), || Triviality::Nontrivial(generator())));

        let first_triad = crate::select(
            Clause::Question,
            &core::iter::chain(
                polymorphic_trivial_choices.each_ref()
                    .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> Triviality<T, N>)),
                polymorphic_nontrivial_choices.each_ref()
                    .map(|(choice, generator)| (*choice, generator as &dyn Fn() -> Triviality<T, N>))
            ).collect::<Vec<_>>()
        );
        let (trivial_triad, nontrivial_triad) = match first_triad
        {
            Triviality::Trivial(trivial_triad) => {
                (
                    trivial_triad,
                    crate::select(
                        Clause::Continuation(nontrivial_conjunction),
                        &nontrivial_choices.each_ref()
                            .map(|(choice, generator)| (choice.expression.as_ref(), generator as &dyn Fn() -> N))
                    )
                )
            },
            Triviality::Nontrivial(nontrivial_triad) => {
                (
                    crate::select(
                        Clause::Continuation(trivial_conjunction),
                        &trivial_choices.each_ref()
                            .map(|(choice, generator)| (choice.expression.as_ref(), generator as &dyn Fn() -> T))
                    ),
                    nontrivial_triad
                )
            },
        };
        let domain = trivial_triad + nontrivial_triad;
        assert_eq!(domain.kind(config, fallback), domain_kind, "Domain-kind must be invariant! (it isn't)");
        domain
    }

    let domain = crate::select::<Box<dyn Domain>>(
        Clause::Answer("please select a domain"),
        &[
            (InternalDissonance::kind(config, fallback), &|| Box::new(select_triads(", but ", Frame::all(), ", but ", Means::all(), config, fallback))),
            (InternalSynthesis::kind(config, fallback), &|| Box::new(select_triads(", but ", Frame::all(), ", ", Fault::all(), config, fallback))),
            (DesireMachine::kind(config, fallback), &|| Box::new(select_triads(", ", Frame::all(), " and ", Need::all(), config, fallback))),
            (BodyWithoutOrgans::kind(config, fallback), &|| Box::new(select_triads(", ", Fault::all(), " and ", Means::all(), config, fallback))),
            (ExternalSynthesis::kind(config, fallback), &|| Box::new(select_triads(", but ", Need::all(), ", ", Means::all(), config, fallback))),
            (ExternalDissonance::kind(config, fallback), &|| Box::new(select_triads(", but ", Need::all(), ", but ", Fault::all(), config, fallback))),
        ]
    );
    let answer = core::fmt::from_fn(|f| domain.answer(f, config, fallback));
    println!("A: {answer}");

    domain
}

pub fn select_pivot(pivot: Pivot, config: &(impl Property<EnneagramConfig> + ?Sized), fallback: &Fallback) -> Enneatype
{
    let config = config.property(fallback);

    let h = pivot.homeostatis().config(config, fallback);
    let question = h.pivot.as_ref();

    crate::select(
        Clause::Answer(question),
        &[pivot.extroverted(), pivot.homeostatis(), pivot.introverted()]
            .map(|edge| {
                let affirmation = core::fmt::from_fn(|f| edge.affirmation(f, config, fallback));
                (format!("{affirmation}"), move || edge)
            })
            .each_ref()
            .map(|(affirmation, generator)| (&**affirmation, generator as &dyn Fn() -> Enneatype))
    )
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
