use std::{fmt, fmt::Display};

struct Wrap<T>(T);

impl<T: Display> Display for Wrap<Vec<T>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (ix, r) in self.0.iter().enumerate() {
            if ix > 0 {
                write!(f, " ")?;
            }
            r.fmt(f)?;
        }
        Ok(())
    }
}

#[argio::argio]
fn main(n: usize) -> Wrap<Vec<usize>> {
    Wrap((0..n).map(|i| i * 2).collect())
}
