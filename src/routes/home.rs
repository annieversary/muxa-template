use super::*;

pub async fn home(
    _: HomePath,
    Extension(html): Extension<HtmlContextBuilder>,
    Extension(_session): Extension<UserSession>,
) -> impl IntoResponse {
    let content = html! {
        h1 {
            "muxa!"
        }
        p."m[0]" {
            "welcome to muxa, this is a template"
        }
    };

    Ok::<_, ErrResponse>(html.build(content))
}
