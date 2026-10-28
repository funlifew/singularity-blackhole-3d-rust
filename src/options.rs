// Native CLI options are unavailable in a browser build.
#![cfg_attr(target_arch = "wasm32", allow(dead_code))]

use crate::{camera::OrbitCamera, settings::Quality};

pub const HELP: &str = "Singularity - interactive black hole explorer

Usage: singularity-rust [OPTIONS]

  --preset NAME     cinematic (default), edge-on, top-down, or jets
  --quality LEVEL   low, medium (default), or high
  --width PIXELS    window width, 320..7680 (default: 1280)
  --height PIXELS   window height, 240..4320 (default: 760)
  --time SECONDS    start at a fixed simulation time
  --no-ui           hide the HUD and controls
  --no-lensing      trace straight rays for comparison
  --capture PATH    save a PNG and exit (native desktop only)
  --frames COUNT    frames before capture, 1..600 (default: 3)
  -h, --help        print help
  -V, --version     print version
";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    #[default]
    Cinematic,
    EdgeOn,
    TopDown,
    Jets,
}
impl Preset {
    pub fn camera(self) -> OrbitCamera {
        match self {
            Self::Cinematic | Self::Jets => OrbitCamera::default(),
            Self::EdgeOn => OrbitCamera::at(0.8, 0.055, 23.0),
            Self::TopDown => OrbitCamera::at(0.8, 1.28, 26.0),
        }
    }
}

#[derive(Debug)]
pub struct Options {
    pub width: i32,
    pub height: i32,
    pub time: f32,
    pub no_ui: bool,
    pub no_lensing: bool,
    pub quality: Quality,
    pub preset: Preset,
    pub capture: Option<String>,
    pub frames: u32,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 760,
            time: 0.0,
            no_ui: false,
            no_lensing: false,
            quality: Quality::default(),
            preset: Preset::default(),
            capture: None,
            frames: 3,
        }
    }
}
impl Options {
    pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut options = Self::default();
        let mut args = args.into_iter();
        while let Some(flag) = args.next() {
            if flag == "--no-lensing" {
                options.no_lensing = true;
                continue;
            }
            if flag == "--no-ui" {
                options.no_ui = true;
                continue;
            }
            if !matches!(
                flag.as_str(),
                "--width"
                    | "--height"
                    | "--time"
                    | "--quality"
                    | "--preset"
                    | "--capture"
                    | "--frames"
            ) {
                return Err(format!("unknown option '{flag}'"));
            }
            let value = args
                .next()
                .ok_or_else(|| format!("missing value for {flag}"))?;
            match flag.as_str() {
                "--width" => options.width = bounded(&value, 320, 7680, &flag)?,
                "--height" => options.height = bounded(&value, 240, 4320, &flag)?,
                "--frames" => options.frames = bounded(&value, 1, 600, &flag)? as u32,
                "--quality" => options.quality = value.parse()?,
                "--capture" => {
                    if !value.to_ascii_lowercase().ends_with(".png") {
                        return Err("--capture requires a .png path".into());
                    }
                    options.capture = Some(value);
                }
                "--time" => {
                    options.time = value.parse().map_err(|_| "--time requires a number")?;
                    if !options.time.is_finite() || options.time < 0.0 {
                        return Err("--time must be finite and nonnegative".into());
                    }
                }
                "--preset" => {
                    options.preset = match value.as_str() {
                        "cinematic" => Preset::Cinematic,
                        "edge-on" => Preset::EdgeOn,
                        "top-down" => Preset::TopDown,
                        "jets" => Preset::Jets,
                        _ => return Err(format!("unknown preset '{value}'")),
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(options)
    }

    pub fn from_env() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let args: Vec<String> = std::env::args().skip(1).collect();
            if args.iter().any(|a| a == "--help" || a == "-h") {
                println!("{HELP}");
                std::process::exit(0);
            }
            if args.iter().any(|a| a == "--version" || a == "-V") {
                println!("singularity-rust {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Self::parse(args).unwrap_or_else(|error| {
                eprintln!("{error}\n\n{HELP}");
                std::process::exit(2)
            })
        }
        #[cfg(target_arch = "wasm32")]
        Self::default()
    }
}
fn bounded(value: &str, min: i32, max: i32, flag: &str) -> Result<i32, String> {
    let number: i32 = value
        .parse()
        .map_err(|_| format!("{flag} requires an integer"))?;
    if !(min..=max).contains(&number) {
        return Err(format!("{flag} must be between {min} and {max}"));
    }
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn parse(args: &[&str]) -> Result<Options, String> {
        Options::parse(args.iter().map(|s| s.to_string()))
    }
    #[test]
    fn invalid_capture_parameters_are_rejected() {
        for args in [
            vec!["--time", "NaN"],
            vec!["--time", "-1"],
            vec!["--time", "inf"],
            vec!["--width", "0"],
            vec!["--height", "99999"],
            vec!["--frames", "0"],
            vec!["--capture", "image.jpg"],
            vec!["--quality", "ultra"],
            vec!["--preset", "invalid"],
            vec!["--width"],
            vec!["--unknown"],
        ] {
            assert!(parse(&args).is_err(), "{args:?}");
        }
    }
    #[test]
    fn deterministic_capture_options_parse() {
        let options = parse(&[
            "--preset",
            "edge-on",
            "--time",
            "12.5",
            "--no-ui",
            "--capture",
            "out.png",
            "--quality",
            "high",
        ])
        .unwrap();
        assert_eq!(options.preset, Preset::EdgeOn);
        assert_eq!(options.time, 12.5);
        assert!(options.no_ui);
        assert_eq!(options.quality, Quality::High);
        assert_eq!(options.capture.as_deref(), Some("out.png"));
    }
}
