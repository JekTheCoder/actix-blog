use actix_web::HttpRequest;

pub fn bearer(req: &HttpRequest) -> Option<&str> {
    let header = req.headers().get("Authorization")?;
    let header_content = header.to_str().ok()?;

    let mut split = header_content.split(' ');
    let may_bearer = split.next()?;

    match may_bearer {
        "Bearer" => split.next(),
        _ => None,
    }
}
