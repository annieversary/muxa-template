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
        p {
            "welcome to muxa, this is a template"
        }
    };

    Ok::<_, ErrResponse>(html.build(content))
}
