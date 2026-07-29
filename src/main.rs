use std::{fmt::Display, io::Write, ops::Add};

#[cfg(feature = "blasphemy")]
use rand::distr::Distribution;
#[cfg(feature = "blasphemy")]
use std::io::Read;

use enneagram::{Enneagram, Enneatype, config::{Config, EnneagramConfig, Fallback, Property}, domain::{BodyWithoutOrgans, DesireMachine, Domain, ExternalDissonance, ExternalSynthesis, InternalDissonance, InternalSynthesis}, pivot::Pivot, triad::{Fault, Frame, Means, Need, Triad}};

use crate::artwork::Artwork;

moddef::moddef!(
    mod {
        artwork for cfg(feature = "artwork")
    }
);

fn main() -> Result<(), ProgramError>
{
    match (|| {
        Program::from_args(std::env::args().skip(1))?.run()?;

        Ok(())
    })()
    {
        Ok(()) => Ok(()),
        Err(error) => {
            println!("{error}");
            Err(error)
        }
    }
}

#[derive(Debug)]
enum ProgramError
{
    Argument(ArgumentError),
    Expectation(ExpectationError)
}
impl Display for ProgramError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::Argument(error) => error.fmt(f),
            Self::Expectation(error) => error.fmt(f) 
        }
    }
}
impl From<ArgumentError> for ProgramError
{
    fn from(error: ArgumentError) -> Self
    {
        Self::Argument(error)
    }
}
impl From<ExpectationError> for ProgramError
{
    fn from(error: ExpectationError) -> Self
    {
        Self::Expectation(error)
    }
}

#[derive(Debug)]
enum ArgumentError
{
    #[cfg(feature = "pivot")]
    PivotAlreadyEnabled,
    #[cfg(feature = "pivot")]
    PivotAlreadyDisabled,
    #[cfg(feature = "artwork")]
    ArtworkAlreadyEnabled,
    #[cfg(feature = "artwork")]
    ArtworkAlreadyDisabled,
    ExpectedFlag,
    InvalidArgument {
        argument: String
    },
    UnrecognizedFlag {
        flag: String
    },
    UnrecognizedSingleCharacterFlag {
        flag: char
    }
}

impl Display for ArgumentError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::PivotAlreadyEnabled => write!(f, "Pivot is already enabled"),
            Self::PivotAlreadyDisabled => write!(f, "Pivot is already disabled"),
            Self::ArtworkAlreadyEnabled => write!(f, "Artwork is already enabled"),
            Self::ArtworkAlreadyDisabled => write!(f, "Artwork is already disabled"),
            Self::ExpectedFlag => write!(f, "Invalid argument: Expected flag"),
            Self::InvalidArgument { argument } => write!(f, "Invalid arguments. Didn't expect '{argument}'."),
            Self::UnrecognizedFlag { flag } => write!(f, "Invalid argument: Unrecognized flag '{flag}'"),
            Self::UnrecognizedSingleCharacterFlag { flag } => write!(f, "Invalid argument: Unrecognized single-character flag '{flag}'")
        }
    }
}

#[derive(Debug)]
enum ExpectationError
{
    ExpectedConfig,
    ExpectedFallbackConfig,
}

impl Display for ExpectationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Self::ExpectedConfig => write!(f, "Expected argument: config-file (yaml, see {}).", Config::default_config_path().to_string_lossy()),
            Self::ExpectedFallbackConfig => write!(f,
                "Expected argument. Implied additional fallback config-file (yaml, see '{}'), due to preceding ':'-operator.",
                Config::default_config_path().to_string_lossy()
            )
        }
    }
}

enum Expectation
{
    Config,
    MaybeColon,
    FallbackConfig
}

enum Flag
{
    #[cfg(feature = "pivot")]
    Pivot,
    #[cfg(feature = "artwork")]
    Artwork,
    Config
}

#[derive(Default)]
struct Program
{
    #[cfg(feature = "pivot")]
    enable_pivot: bool,
    #[cfg(feature = "artwork")]
    enable_artwork: bool,
    configs: Vec<(String, Vec<Config>)>,
    enneagram: Enneagram,
    expectation: Option<Expectation>
}

impl Program
{
    pub fn run(self) -> Result<(), ExpectationError>
    {
        match self.expectation
        {
            Some(Expectation::Config) => return Err(ExpectationError::ExpectedConfig),
            None | Some(Expectation::MaybeColon) => (),
            Some(Expectation::FallbackConfig) => return Err(ExpectationError::ExpectedFallbackConfig)
        }

        let (config, fallback) = self.configuration();
        
        #[cfg(feature = "artwork")]
        if self.enable_artwork
        { 
            let artwork = Artwork {
                enneagram: &self.enneagram,
                config: &config,
                fallback: &fallback
            };

            let mut terminal = ratatui::init();
            artwork.draw(&mut terminal);

            return Ok(())
        }

     
        if !self.enneagram.is_empty()
        {
            let mut sep = "";
            for edges in self.enneagram.edges()
            {
                let edge_info = core::fmt::from_fn(|f| Enneatype::common_info(edges, f, &config, &fallback));
                println!("{sep}{edge_info}");
                sep = "\n"
            }
            return Ok(())
        }

        let domain = select_domain(&config, &fallback);
        let mut edge = domain.edge();

        let edge_info = core::fmt::from_fn(|f| edge.info(f, &config, &fallback));
        println!("\n{edge_info}");

        #[cfg(feature = "pivot")]
        if self.enable_pivot
        {
            loop
            {
                println!();
                let pivot = edge.pivot();
                let origin = core::mem::replace(&mut edge, select_pivot(pivot, &config, &fallback));
                if edge == origin
                {
                    break
                }

                let edge_info = core::fmt::from_fn(|f| edge.info(f, &config, &fallback));
                println!("\n{edge_info}");
            }
        }

        Ok(())
    }
    
    pub fn from_args(arguments: impl IntoIterator<Item: Into<String>>) -> Result<Self, ArgumentError>
    {
        let mut program = Self::default();
        for argument in arguments
        {
            program.take_arg(argument.into())?
        }
        Ok(program)
    }

    fn take_arg(&mut self, argument: String) -> Result<(), ArgumentError>
    {
        match self.expectation.take()
        {
            Some(Expectation::Config) => {
                let config = Config::read_config(&argument);
                self.configs.push((argument, vec![config]));
                self.expectation = Some(Expectation::MaybeColon);
                return Ok(())
            },
            Some(Expectation::MaybeColon) => {
                if argument.trim() == ":"
                {
                    self.expectation = Some(Expectation::FallbackConfig);
                    return Ok(())
                }
            },
            Some(Expectation::FallbackConfig) => {
                self.configs.last_mut()
                    .expect("There should already be a configuration to override.")
                    .1
                    .push(Config::read_config(&argument));
                return Ok(())
            },
            None => ()
        }

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
                return Err(ArgumentError::ExpectedFlag)
            }
            let flag = match flag_str
            {
                #[cfg(feature = "pivot")]
                "pivot" => Flag::Pivot,
                #[cfg(feature = "artwork")]
                "artwork" => Flag::Artwork,
                "config" => Flag::Config,
                _ => return Err(ArgumentError::UnrecognizedFlag {
                    flag: flag_str.to_string()
                })
            };
            self.take_flag(
                flag,
                std::mem::replace(&mut invert, false)
            )?;
            return Ok(())
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
                    _ => return Err(ArgumentError::UnrecognizedSingleCharacterFlag {
                        flag: flag_char
                    })
                };
                self.take_flag(
                    flag,
                    std::mem::replace(&mut invert, false)
                )?;
            }
            if invert
            {
                return Err(ArgumentError::ExpectedFlag)
            }
            return Ok(())
        }
        else if let Ok(mut number) = argument.parse::<u128>().map(Some)
        {
            self.enneagram.push_edges(
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
            return Ok(())
        }
        Err(ArgumentError::InvalidArgument { argument })
    }

    fn take_flag(&mut self, flag: Flag, invert: bool) -> Result<(), ArgumentError>
    {
        match flag
        {
            #[cfg(feature = "pivot")]
            Flag::Pivot => match (self.enable_pivot, invert)
            {
                (true, true) => self.enable_pivot = false,
                (true, false) => return Err(ArgumentError::PivotAlreadyEnabled),
                (false, true) => return Err(ArgumentError::PivotAlreadyDisabled),
                (false, false) => self.enable_pivot = true
            },
            #[cfg(feature = "artwork")]
            Flag::Artwork => match (self.enable_artwork, invert)
            {
                (true, true) => self.enable_artwork = false,
                (true, false) => return Err(ArgumentError::ArtworkAlreadyEnabled),
                (false, true) => return Err(ArgumentError::ArtworkAlreadyDisabled),
                (false, false) => self.enable_artwork = true
            },
            Flag::Config => if invert
            {
                self.configs.clear()
            }
            else
            {
                assert!(self.expectation.is_none(), "There should be no expectations!");
                self.expectation = Some(Expectation::Config)
            }
        }
        Ok(())
    }

    pub fn configuration(&self) -> (Config, Fallback)
    {
        let mut fallback = Fallback::default();

        let configs = match self.configs.len()
        {
            0 => return (Default::default(), fallback),
            1 => {
                let [(_, config)] = self.configs.as_slice()
                    .as_array()
                    .expect("Config should now be unambiguous.");
                config.as_slice()
            },
            2.. => {
                let options = self.configs.iter()
                    .map(|config| (config.0.as_str(), move || config.1.as_slice()))
                    .collect::<Vec<_>>();

                crate::select::<&[Config]>(
                    Clause::Answer("please select one config"),
                    &options.iter()
                        .map(|config| (config.0, &config.1 as &dyn Fn() -> _))
                        .collect::<Vec<_>>()
                )
            }
        };

        let config = configs.iter()
            .rev()
            .cloned()
            .reduce(|mut a, b| {
                a.overlay_config(b, &mut fallback);
                a
            }).unwrap_or_default();

        (config, fallback)
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

    use crate::{Program, ProgramError};

    #[cfg(feature = "artwork")]
    #[test]
    #[ignore] // It fucks up the keyboard input, because i'm not fluent in ratatui
    fn test_color_override() -> Result<(), ProgramError>
    {
        const YAML: &str = "/tmp/test_color_override_enneagram.yaml";

        std::fs::write(Path::new(YAML), "color:\n  glare: FF00FF\n  sun: FFFF00").unwrap();

        Program::from_args(["-ac", YAML])?.run()?;

        Ok(())
    }

    #[test]
    fn test_christ_enneagram() -> Result<(), ProgramError>
    {
        const YAML: &str = "/tmp/chist_enneagram.yaml";

        std::fs::write(Path::new(YAML), include_str!("../presets/christ.yaml")).unwrap();

        Program::from_args(["-c", YAML, "1"])?.run()?;

        Ok(())
    }
}
