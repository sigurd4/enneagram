use crate::{Clause, enneatype::Enneatype};

#[derive(Debug, Clone, Copy)]
pub struct Pivot
{
    question: &'static str,
    extroverted: Enneatype,
    homeostatis: Enneatype,
    introverted: Enneatype
}

impl Pivot
{
    pub fn new(edge: Enneatype) -> Self
    {
        use Enneatype::*;

        let (question, extroverted, introverted) = match edge
        {
            Recovery => ("how will you manage your frustration?", Disorganization, Rejection), // 714
            Association => ("how will you gain worth?", Action, Rejection), // 824
            Repression => ("how will you repress your shame?", Rest, Paranoia), // 936
            Rejection => ("how will you deal with your longing?", Association, Recovery), // 241
            Catatonia => ("how will you gain security?", Action, Disorganization), // 857
            Paranoia => ("how will you deal with your insecurity?", Repression, Rest), // 369
            Disorganization => ("how will you handle your fear?", Catatonia, Recovery), // 571
            Action => ("how will you gain control?", Catatonia, Association), // 582 
            Rest => ("how will you suppress your anger?", Paranoia, Repression), // 693
        };
        Pivot { question, extroverted, homeostatis: edge, introverted }
    }

    pub fn extroverted(&self) -> Enneatype
    {
        //assert_eq!(self.extroverted.pivot().introverted, self.homeostatis);
        self.extroverted
    }
    pub fn introverted(&self) -> Enneatype
    {
        //assert_eq!(self.introverted.pivot().extroverted, self.homeostatis);
        self.introverted
    }
    pub fn is_adjacent_to(&self, edge: Enneatype) -> bool
    {
        self.extroverted == edge || self.introverted == edge
    }

    pub fn select(self) -> Enneatype
    {
        crate::select(
            Clause::Answer(self.question),
            &[self.extroverted, self.homeostatis, self.introverted]
                .map(|edge| {
                    let affirmation = core::fmt::from_fn(|f| edge.affirmation(f));
                    (format!("{affirmation}"), move || edge)
                }).each_ref()
                .map(|(affirmation, generator)| (&**affirmation, generator as &dyn Fn() -> Enneatype))
        )
    }

    pub fn lines(self) -> [[Enneatype; 2]; 2]
    {
        let Self { question: _, extroverted, homeostatis, introverted } = self;
        [[extroverted, homeostatis], [homeostatis, introverted]]
    }
}