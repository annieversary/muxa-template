use std::convert::Infallible;

use axum::extract::{FromRequest, RequestParts};
use maud::{html, Markup};

use crate::{routes::*, theme::Theme};
use muxa::{
    html::{components::*, *},
    theme::*,
};

pub type HtmlContextBuilder = muxa::html::HtmlContextBuilder<InternalContext, NamedRoute>;

#[derive(Clone)]
pub struct InternalContext {
    theme: Theme,
}

#[axum::async_trait]
impl<B> FromRequest<B> for InternalContext
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let ThemeCookie(theme) = ThemeCookie::from_request(req).await.unwrap();
        Ok(InternalContext { theme })
    }
}

pub const ASSET_V: u16 = muxa::reexports::const_random!(u16);

impl Template<NamedRoute> for InternalContext {
    fn head(ctx: &HtmlContext<Self, NamedRoute>) -> Markup {
        let title = ctx.get_title();
        let description = ctx.get_description().unwrap_or("muxa template :)");

        html! {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            @stylesheet(ctx.inner.theme.css_url(), ASSET_V);
            @stylesheet("/css/custom.css", ASSET_V);
            @stylesheet("/css/zephyr.css", ASSET_V);
            (ctx.section_get("styles"))

            meta property="og:title" content=(title);
            meta property="og:description" content=(description);
            meta property="description" content=(description);
            meta name="author" content="your name here !";
            meta name="theme-color" content="#eaacb8";

            @if let Some(i) = ctx.get_image() {
                meta property="og:image" content=(i);
                meta name="twitter:card" content="summary_large_image";
            }

            title {(title)}
        }
    }

    fn body(ctx: &HtmlContext<Self, NamedRoute>) -> Markup {
        html! {
            main {
                (ctx.content)
            }

            (ctx.section_get("scripts"))
        }
    }
}
