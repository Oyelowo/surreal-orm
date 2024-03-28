use std::fmt::Display;

/// Direction of the connection edge
#[derive(Debug, Clone, Copy)]
pub enum EdgeDirection {
    /// Outgoing edge
    Out,
    /// Incoming edge
    In,
}

impl EdgeDirection {
    /// Get the arrow symbol for the edge direction
    pub fn as_arrow_symbol(&self) -> Arrow {
        match self {
            EdgeDirection::Out => Arrow::Right,
            EdgeDirection::In => Arrow::Left,
        }
    }
}

/// Arrow for relation edge direction
#[derive(Debug, Clone, Copy)]
pub enum Arrow {
    /// Right arrow
    Right,
    /// Left arrow
    Left,
}

impl Display for Arrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arrow_str = match self {
            Arrow::Right => "->",
            Arrow::Left => "<-",
        };
        write!(f, "{arrow_str}")
    }
}
