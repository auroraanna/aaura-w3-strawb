use axum::{
    body::{
        Body,
        to_bytes
    },
    extract,
    middleware::Next,
    response::IntoResponse
};
use http::{
    header::{
        ETAG,
        LAST_MODIFIED,
        IF_NONE_MATCH,
        CONTENT_TYPE
    },
    HeaderValue,
    StatusCode
};
use xxhash_rust::xxh3::xxh3_128;
use base64::{
    engine::general_purpose::STANDARD_NO_PAD,
    Engine as _
};

pub async fn apply_etag(req: extract::Request, next: Next) -> impl IntoResponse {
    let (req_parts, req_body) = req.into_parts();
    let if_none_match = req_parts.headers.get(IF_NONE_MATCH);
    let req = extract::Request::from_parts(req_parts.clone(), req_body);
    let (mut res_parts, res_body) = next.run(req).await.into_parts();

    res_parts.headers.remove(LAST_MODIFIED);

    // HTML changes every request due to nonces so etags are unnescessary
    if !res_parts.status.is_success() || res_parts.headers.get(CONTENT_TYPE).unwrap() == "text/html; charset=utf-8" {
        return (
            res_parts,
            res_body
        );
    }

    let res_body_bytes = to_bytes(res_body, usize::MAX).await.unwrap();
    let res_body_hash = xxh3_128(&res_body_bytes);
    let res_body_hash_base64 = STANDARD_NO_PAD.encode(res_body_hash.to_be_bytes());
    let quoted_res_body_hash_base64 = format!(
        "\"{}\"",
        res_body_hash_base64
    );

    res_parts.headers.insert(
        ETAG,
        HeaderValue::from_str(&quoted_res_body_hash_base64).unwrap()
    );

    match if_none_match {
        Some(inm) => {
            if inm.to_str().unwrap().strip_prefix("\"").unwrap().strip_suffix("\"").unwrap() == res_body_hash_base64 {
                res_parts.status = StatusCode::NOT_MODIFIED;
                return (
                    res_parts,
                    Body::from("")
                )
            }
        },
        None => {}
    }

    return (
        res_parts,
        Body::from(res_body_bytes)
    )
}
