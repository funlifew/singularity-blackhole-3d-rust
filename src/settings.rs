use std::str::FromStr;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Quality {
    Low,
    #[default]
    Medium,
    High,
}
impl Quality {
    pub fn scale(self) -> f32 {
        match self {
            Self::Low => 0.5,
            Self::Medium => 0.75,
            Self::High => 1.0,
        }
    }
    pub fn steps(self) -> f32 {
        match self {
            Self::Low => 150.0,
            Self::Medium => 210.0,
            Self::High => 260.0,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::Low => Self::Medium,
            Self::Medium => Self::High,
            Self::High => Self::Low,
        }
    }
}
impl FromStr for Quality {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "low" => Ok(Self::Low),
            "medium" => Ok(Self::Medium),
            "high" => Ok(Self::High),
            _ => Err(format!(
                "invalid quality '{value}'; use low, medium, or high"
            )),
        }
    }
}

pub struct Settings {
    pub speed: f32,
    pub density: f32,
    pub exposure: f32,
    pub bloom: f32,
    pub lensing: bool,
    pub jets: bool,
    pub paused: bool,
    pub show_ui: bool,
    pub auto_orbit: bool,
    pub quality: Quality,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            density: 0.8,
            exposure: 1.25,
            bloom: 0.22,
            lensing: true,
            jets: false,
            paused: false,
            show_ui: true,
            auto_orbit: false,
            quality: Quality::default(),
        }
    }
}
