pub fn corner<P>(l1: [P; 2], l2: [P; 2]) -> Option<[P; 3]>
where
    P: PartialEq
{
    match (l1, l2)
    {
        ([p, a] | [a, p], [p_eq, b] | [b, p_eq]) if p == p_eq && a != b => Some([a, p, b]),
        _ => None
    }
}

pub fn equals<P>(l1: &[P; 2], l2: &[P; 2]) -> bool
where
    P: PartialEq
{
    match (l1, l2)
    {
        ([a1, b1], [a2, b2] | [b2, a2]) if a1 == a2 && b1 == b2 => true,
        _ => false
    }
}