use anyhow::{anyhow, Context, Result};
use calm_io::stdoutln;
use itertools::Itertools;
use std::fs::read_to_string;
use std::path::Path;
use tinted_builder::{Base16Scheme, Color};

use crate::find::find_schemes;

fn true_color(hex_color: &str, background: bool) -> Result<String> {
    let rgb = hex::decode(hex_color)?;

    let code = if background { 48 } else { 38 };

    Ok(format!("\x1b[{};2;{};{};{}m", code, rgb[0], rgb[1], rgb[2]))
}

pub fn print_color(color: &str) -> Result<()> {
    const RESETCOLOR: &str = "\x1b[0m";
    match stdoutln!(
        "{} #{} {}  {}#{}{}",
        true_color(color, true)?,
        color,
        RESETCOLOR,
        true_color(color, false)?,
        color,
        RESETCOLOR
    ) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

pub fn print_color_rgb(color: Color) -> Result<()> {
    use std::fmt::{self, Display, Formatter};

    struct TrueColor(Color, bool);

    impl Display for TrueColor {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let (r, g, b) = self.0.rgb;
            let code = if self.1 { 48 } else { 38 };
            write!(f, "\x1b[{code};2;{r};{g};{b}m")
        }
    }

    const RESETCOLOR: &str = "\x1b[0m";

    let true_color_fg = TrueColor(color.clone(), true);
    let true_color_bg = TrueColor(color.clone(), false);

    match stdoutln!("{true_color_fg} #{color} {RESETCOLOR}  {true_color_bg}#{color}{RESETCOLOR}",) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::BrokenPipe => Ok(()),
            _ => Err(e),
        },
    }?;
    Ok(())
}

/// Info subcommand
///
/// * `patterns` - Vector with patterns
/// * `base_dir` - flavours base data dir
/// * `verbose` - Should we be verbose? (unused)
/// * `color` - Should we print with colors?
pub fn info(patterns: Vec<&str>, base_dir: &Path, config_dir: &Path, raw: bool) -> Result<()> {
    let mut schemes = Vec::new();
    for pattern in patterns {
        let found_schemes = find_schemes(pattern, base_dir, config_dir)?;

        schemes.extend_from_slice(&found_schemes)
    }
    schemes.sort();
    schemes.dedup();

    if schemes.is_empty() {
        return Err(anyhow!("No matching scheme found"));
    };

    let mut first = true;
    for scheme_file in schemes {
        if first {
            first = false;
        } else {
            match stdoutln!() {
                Ok(_) => Ok(()),
                Err(e) => match e.kind() {
                    std::io::ErrorKind::BrokenPipe => Ok(()),
                    _ => Err(e),
                },
            }?;
        }
        let scheme_contents = read_to_string(&scheme_file)
            .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

        let scheme: Base16Scheme = serde_yaml::from_str(&scheme_contents)?;

        match stdoutln!(
            "{} ({}) @ {}",
            scheme.name,
            scheme.slug,
            scheme_file.to_string_lossy()
        ) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        match stdoutln!("by {}", scheme.author) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::BrokenPipe => Ok(()),
                _ => Err(e),
            },
        }?;

        if raw {
            for color in scheme.palette.iter().sorted_by_key(|x| x.0) {
                match stdoutln!("#{}", color.1.to_hex()) {
                    Ok(_) => Ok(()),
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::BrokenPipe => Ok(()),
                        _ => Err(e),
                    },
                }?;
            }
        } else {
            for color in scheme.palette.iter().sorted_by_key(|x| x.0) {
                print_color_rgb(color.1.clone())?;
            }
        }
    }

    Ok(())
}
