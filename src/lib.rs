mod info;

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use regex_macro::regex;

pub use info::get_entries;

pub fn register_helpers(registry: &mut Handlebars) {
    let closure = move |helper: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
          -> HelperResult {
        let Some(param) = helper.param(0) else {
            return Err(RenderError::new("`id` helper: Param not given"));
        };

        out.write(&text_as_id(&param.render()))?;

        Ok(())
    };
    registry.register_helper("id", Box::new(closure));

    let closure = move |helper: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
          -> HelperResult {
        let Some(a) = helper.param(0) else {
            return Err(RenderError::new("`arrays-intersect` helper: First param not given"));
        };
        let Some(a) = a.value().as_array() else {
            return Err(RenderError::new("`arrays-intersect` helper: First param must be of type `array`"));
        };

        let Some(b) = helper.param(1) else {
            return Err(RenderError::new("`arrays-intersect` helper: Second param not given"));
        };
        let Some(b) = b.value().as_array() else {
            return Err(RenderError::new("`arrays-intersect` helper: Second param must be of type `array`"));
        };

        for i in a {
            if a != b && b.contains(i) {
                out.write("true")?;
            }
        }

        Ok(())
    };
    registry.register_helper("arrays-intersect", Box::new(closure));

    let closure = move |helper: &Helper,
                        _: &Handlebars,
                        _: &Context,
                        _: &mut RenderContext,
                        out: &mut dyn Output|
          -> HelperResult {
        let Some(url) = helper.param(0) else {
            return Err(RenderError::new("`url-domain` helper: Param not given"));
        };
        let url = url.render();

        let Some(captures) =
            regex!(r"(?:https?://)?(?:www\.)?([\w-]+(?:\.[\w-]+)+)")
            .captures(&url)
        else {
            return Err(RenderError::new(format!("`url-domain` helper: String is not valid URL: '{}'", url)));
        };

        out.write(captures.get(1).unwrap().as_str())?;

        Ok(())
    };
    registry.register_helper("url-domain", Box::new(closure));
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
