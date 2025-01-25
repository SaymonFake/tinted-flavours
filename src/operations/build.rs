use anyhow::Ok;
use anyhow::{Context, Result};
use std::fs;
use std::path;
use tinted_builder::{Base16Scheme, Scheme, SchemeSystem, Template};

/// Build a template
///
/// Given template base and scheme, builds the template and returns it
///
/// * `template_base` - Template base string
/// * `scheme` - Scheme structure
pub fn build_template(template_base: &str, scheme: &Base16Scheme) -> Result<String> {
    let template = Template::new(template_base.to_string(), convert_scheme(scheme.clone()));
    Ok(template.render()?)
}

/// Build function
///
/// * `scheme_file` - Path to scheme file
/// * `template_file` - Path to template
pub fn build(scheme_file: &path::Path, template_file: &path::Path) -> Result<()> {
    //Read chosen scheme
    let scheme_contents = &fs::read_to_string(&scheme_file)
        .with_context(|| format!("Couldn't read scheme file at {:?}.", scheme_file))?;

    let scheme: Base16Scheme = serde_yaml::from_str(scheme_contents)?;

    //Template content
    let template_content = fs::read_to_string(template_file)
        .with_context(|| format!("Couldn't read template file at {:?}.", template_file))?;

    let template = Template::new(template_content, convert_scheme(scheme));

    //Template with correct colors
    println!("{}", template.render()?);
    Ok(())
}

/// Convert scheme
///
/// Convert a scheme to be used by the template builder
///
/// * `scheme` - Scheme structure
fn convert_scheme(mut scheme: Base16Scheme) -> Scheme {
    // add fallback colors to base16 schemes
    if scheme.system == SchemeSystem::Base16 {
        let palette = &mut scheme.palette;

        palette.insert("base10".to_string(), palette["base00"].clone());
        palette.insert("base11".to_string(), palette["base00"].clone());
        palette.insert("base12".to_string(), palette["base08"].clone());
        palette.insert("base13".to_string(), palette["base0A"].clone());
        palette.insert("base14".to_string(), palette["base0B"].clone());
        palette.insert("base15".to_string(), palette["base0C"].clone());
        palette.insert("base16".to_string(), palette["base0D"].clone());
        palette.insert("base17".to_string(), palette["base0E"].clone());
    }
    Scheme::Base16(scheme)
}
