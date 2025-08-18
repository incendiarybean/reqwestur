/// An enum containing breakpoint sizes, ordered by size (small..large)
#[derive(PartialEq, PartialOrd)]
pub enum BreakpointSize {
    SMALL = 0,
    MEDIUM = 1,
    LARGE = 2,
    EXTREME = 3,
}

/// Create a Breakpoint Size from an f32
impl From<f32> for BreakpointSize {
    /// Converts from an f32 to a breakpoint
    fn from(size: f32) -> Self {
        match size {
            0.0..600.0 => Self::SMALL,
            600.0..800.0 => Self::MEDIUM,
            800.0..1000.0 => Self::LARGE,
            _ => Self::EXTREME,
        }
    }
}

impl BreakpointSize {
    /// Compares the x/y sizes and creates a BreakpointSize based on the smallest size
    pub fn compare(x: impl Into<Self>, y: impl Into<Self>) -> Self {
        let x = x.into();
        let y = y.into();
        if x < y { x } else { y }
    }
}

/// A struct containing breakpoint information
pub struct Breakpoint {
    x: f32,
    y: f32,
    pub size: BreakpointSize,
}

/// Create a Breakpoint from a tuple of f32s
impl From<(f32, f32)> for Breakpoint {
    /// Convert to a breakpoint from the provided x/y values
    fn from((x, y): (f32, f32)) -> Self {
        let size = BreakpointSize::compare(x, y);

        Self { x, y, size }
    }
}

impl Breakpoint {
    #[allow(dead_code, reason = "May be useful in the future.")]
    /// Get both the x/y Breakpoint Sizes
    pub fn sizes(&self) -> (BreakpointSize, BreakpointSize) {
        (BreakpointSize::from(self.x), BreakpointSize::from(self.y))
    }
}
