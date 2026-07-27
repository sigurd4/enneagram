use crate::enneatype::Enneatype;

#[derive(Debug, Clone)]
pub struct Pivot
{
    extroverted: Enneatype,
    homeostatis: Enneatype,
    introverted: Enneatype
}

impl Pivot
{
    pub fn new(edge: Enneatype) -> Self
    {
        use Enneatype::*;

        let (extroverted, introverted) = match edge
        {
            Recovery => (Disorganization, Rejection), // 714
            Association => (Action, Rejection),       // 824
            Repression => (Rest, Paranoia),           // 936
            Rejection => (Association, Recovery),     // 241
            Catatonia => (Action, Disorganization),   // 857
            Paranoia => (Repression, Rest),           // 369
            Disorganization => (Catatonia, Recovery), // 571
            Action => (Catatonia, Association),       // 582
            Rest => (Paranoia, Repression)            // 693
        };
        Pivot {
            extroverted,
            homeostatis: edge,
            introverted
        }
    }

    pub fn homeostatis(&self) -> Enneatype
    {
        self.homeostatis
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

    pub fn lines(self) -> [[Enneatype; 2]; 2]
    {
        let Self {
            extroverted,
            homeostatis,
            introverted
        } = self;
        [[extroverted, homeostatis], [homeostatis, introverted]]
    }
}
