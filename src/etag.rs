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
	header::ETAG,
	HeaderValue
};
use xxhash_rust::xxh3::xxh3_128;
use base64::{
	engine::general_purpose::STANDARD_NO_PAD,
	Engine as _
};

pub async fn apply_etag(req: extract::Request, next: Next) -> impl IntoResponse {
	let (mut res_parts, res_body) = next.run(req).await.into_parts();
	let res_body_bytes = to_bytes(res_body, usize::MAX).await.unwrap();
	let res_body_hash = xxh3_128(&res_body_bytes);
	let res_body_hash_base64 = STANDARD_NO_PAD.encode(res_body_hash.to_be_bytes());
	let quoted_res_body_hash_hash64 = format!(
		"\"{}\"",
		res_body_hash_base64
	);

	res_parts.headers.insert(
		ETAG,
		HeaderValue::from_str(&quoted_res_body_hash_hash64).unwrap()
	);

	let new_res_body = Body::from(res_body_bytes);

	(
		res_parts,
		new_res_body
	)
}