use std::fmt::Display;

///
#[derive(Clone)]
pub struct OriginHolder<const N: usize> {
  pub segments: [&'static str; N],
}

impl<const N: usize> Display for OriginHolder<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for segment in self.segments {
      write!(f, "{segment}")?;
    }

    Ok(())
  }
}

impl<const N: usize> OriginHolder<N> {
  pub const fn new(segments: [&'static str; N]) -> Self {
    Self { segments }
  }
}
