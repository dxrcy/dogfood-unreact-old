mod info;

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
pub use info::get_entries;

pub fn register_helpers(registry: &mut Handlebars) {
    let closure = move |helper: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
          -> HelperResult {
        let Some(param) = helper.param(0) else {
            return Err(RenderError::new("`id` helper: No param given"))
        };

        out.write(&text_as_id(&param.render()))?;

        Ok(())
    };
    registry.register_helper("id", Box::new(closure));
}

/// Convert text into HTML #id attribute format, with limited character set
pub fn text_as_id(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|ch| {
            if matches!(ch, 'a'..='z'|'0'..='9'|'_'|'-'|'+') {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_as_id_works() {
        assert_eq!(text_as_id("abcDEF1239Zz-_[]!@#a"), "abcdef1239zz-_-----a");
    }
}
