use std::convert::Infallible;

use axum::extract::{FromRequest, RequestParts};
use maud::{html, Markup};

use crate::routes::*;
use muxa::html::*;

pub type HtmlContextBuilder = muxa::html::HtmlContextBuilder<InternalContext, NamedRoute>;

#[derive(Clone)]
pub struct InternalContext {}

#[axum::async_trait]
impl<B> FromRequest<B> for InternalContext
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(_req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(InternalContext {})
    }
}

impl Template<NamedRoute> for InternalContext {
    fn head(ctx: &HtmlContext<Self, NamedRoute>) -> Markup {
        let title = ctx.get_title();
        let description = ctx.get_description().unwrap_or("muxa template :)");

        html! {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";

            // link rel="stylesheet" href="/css/custom.css?v=2";
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
