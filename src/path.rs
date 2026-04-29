pub fn lines<E>(path: impl IntoIterator<Item = E>) -> impl Iterator<Item = [E; 2]>
where
    E: Copy
{
    let path = path.into_iter()
        .collect::<Vec<_>>();
    path.last()
        .copied()
        .map(|mut prev| path.into_iter()
            .map(move |edge| [core::mem::replace(&mut prev, edge), edge])
        ).into_iter()
        .flatten()
}

pub fn lines_disconnected<E>(path: impl IntoIterator<Item = E>) -> impl Iterator<Item = [E; 2]>
where
    E: Copy
{
    let mut path = path.into_iter();
    path.next()
        .map(|mut prev| path.into_iter()
            .map(move |edge| [core::mem::replace(&mut prev, edge), edge])
        ).into_iter()
        .flatten()
}

pub fn length(path: impl IntoIterator<Item = [f64; 2]>) -> f64
{
    crate::path::lines(path)
        .map(|[a, b]| crate::line::length([a, b]))
        .sum::<f64>()
}