//! Measurement units used across Nexivora.

use serde::{Deserialize, Serialize};

/// A physical length, stored in millimetres.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Length(f64);

impl Length {
    /// Create a length from millimetres.
    pub fn from_mm(v: f64) -> Self {
        Self(v.max(0.0))
    }

    /// Create a length from points (1/72 inch).
    pub fn from_pt(v: f64) -> Self {
        Self(v * 25.4 / 72.0)
    }

    /// Create a length from centimetres.
    pub fn from_cm(v: f64) -> Self {
        Self(v * 10.0)
    }

    /// Create a length from inches.
    pub fn from_in(v: f64) -> Self {
        Self(v * 25.4)
    }

    /// Value in millimetres.
    pub fn mm(&self) -> f64 {
        self.0
    }

    /// Value in points.
    pub fn pt(&self) -> f64 {
        self.0 * 72.0 / 25.4
    }

    /// Value in centimetres.
    pub fn cm(&self) -> f64 {
        self.0 / 10.0
    }

    /// Value in inches.
    pub fn inches(&self) -> f64 {
        self.0 / 25.4
    }
}

/// Standard page sizes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaperSize {
    A4,
    A3,
    A5,
    Letter,
    Legal,
    Ledger,
}

impl PaperSize {
    /// The dimensions of this paper size in millimetres (width, height).
    pub fn dimensions_mm(&self) -> (Length, Length) {
        match self {
            PaperSize::A4 => (Length::from_mm(210.0), Length::from_mm(297.0)),
            PaperSize::A3 => (Length::from_mm(297.0), Length::from_mm(420.0)),
            PaperSize::A5 => (Length::from_mm(148.0), Length::from_mm(210.0)),
            PaperSize::Letter => (Length::from_in(8.5), Length::from_in(11.0)),
            PaperSize::Legal => (Length::from_in(8.5), Length::from_in(14.0)),
            PaperSize::Ledger => (Length::from_in(11.0), Length::from_in(17.0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_conversions() {
        let l = Length::from_in(1.0);
        assert!((l.mm() - 25.4).abs() < 1e-9);
        assert!((l.pt() - 72.0).abs() < 1e-9);
        assert!((l.cm() - 2.54).abs() < 1e-9);
    }

    #[test]
    fn a4_dimensions() {
        let (w, h) = PaperSize::A4.dimensions_mm();
        assert!((w.mm() - 210.0).abs() < 1e-9);
        assert!((h.mm() - 297.0).abs() < 1e-9);
    }
}
