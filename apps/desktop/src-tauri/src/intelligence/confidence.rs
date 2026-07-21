use serde::{Deserialize, Serialize};

/// Represents how reliable the current sensor data is.
/// Used by the Decision Engine to gate whether adaptation should occur,
/// and to scale sensitivity of brightness adjustments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfidenceLevel {
    /// Score < 0.10 — sensor absent or completely unreliable. No adaptation.
    VeryLow,
    /// Score 0.10–0.29 — noisy or estimated data. No adaptation.
    Low,
    /// Score 0.30–0.59 — usable data with some uncertainty. Adapt conservatively.
    Medium,
    /// Score 0.60–0.84 — reliable hardware sensor data. Normal adaptation.
    High,
    /// Score 0.85–1.00 — high-quality multi-sample sensor data. Full sensitivity.
    VeryHigh,
}

impl ConfidenceLevel {
    /// Derive confidence level from a raw [0.0, 1.0] score.
    pub fn from_score(score: f32) -> Self {
        if score < 0.10 {
            Self::VeryLow
        } else if score < 0.30 {
            Self::Low
        } else if score < 0.60 {
            Self::Medium
        } else if score < 0.85 {
            Self::High
        } else {
            Self::VeryHigh
        }
    }

    /// Whether adaptation should be applied at this confidence level.
    /// VeryLow and Low are too unreliable to act on.
    pub fn should_adapt(&self) -> bool {
        matches!(self, Self::Medium | Self::High | Self::VeryHigh)
    }

    /// A multiplier [0.0, 1.0] applied to brightness adjustment magnitude.
    /// Prevents over-correcting when confidence is borderline.
    pub fn sensitivity_multiplier(&self) -> f32 {
        match self {
            Self::VeryLow => 0.0,
            Self::Low => 0.0,
            Self::Medium => 0.5,
            Self::High => 0.85,
            Self::VeryHigh => 1.0,
        }
    }

    /// Human-readable label for UI display.
    pub fn label(&self) -> &'static str {
        match self {
            Self::VeryLow => "Very Low",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::VeryHigh => "Very High",
        }
    }

    /// Percentage representation for Developer Mode display.
    pub fn as_percentage(&self, raw_score: f32) -> u8 {
        (raw_score * 100.0).round().clamp(0.0, 100.0) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thresholds() {
        assert_eq!(ConfidenceLevel::from_score(0.0), ConfidenceLevel::VeryLow);
        assert_eq!(ConfidenceLevel::from_score(0.09), ConfidenceLevel::VeryLow);
        assert_eq!(ConfidenceLevel::from_score(0.10), ConfidenceLevel::Low);
        assert_eq!(ConfidenceLevel::from_score(0.30), ConfidenceLevel::Medium);
        assert_eq!(ConfidenceLevel::from_score(0.60), ConfidenceLevel::High);
        assert_eq!(ConfidenceLevel::from_score(0.85), ConfidenceLevel::VeryHigh);
        assert_eq!(ConfidenceLevel::from_score(1.0), ConfidenceLevel::VeryHigh);
    }

    #[test]
    fn test_should_adapt() {
        assert!(!ConfidenceLevel::VeryLow.should_adapt());
        assert!(!ConfidenceLevel::Low.should_adapt());
        assert!(ConfidenceLevel::Medium.should_adapt());
        assert!(ConfidenceLevel::High.should_adapt());
        assert!(ConfidenceLevel::VeryHigh.should_adapt());
    }

    #[test]
    fn test_sensitivity_is_progressive() {
        let medium = ConfidenceLevel::Medium.sensitivity_multiplier();
        let high = ConfidenceLevel::High.sensitivity_multiplier();
        let very_high = ConfidenceLevel::VeryHigh.sensitivity_multiplier();
        assert!(medium < high);
        assert!(high < very_high);
        assert_eq!(very_high, 1.0);
    }
}
