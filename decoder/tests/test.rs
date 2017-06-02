extern crate multipart_decoder;
extern crate futures;
extern crate bytes;

use feature::stream;
use bytes::Bytes;

const test_str = "This is a message with multiple parts in MIME format.
--frontier
Content-Type: text/plain

This is the body of the message.
--frontier
Content-Type: application/octet-stream
Content-Transfer-Encoding: base64

PGh0bWw+CiAgPGhlYWQ+CiAgPC9oZWFkPgogIDxib2R5PgogICAgPHA+VGhpcyBpcyB0aGUg
Ym9keSBvZiB0aGUgbWVzc2FnZS48L3A+CiAgPC9ib2R5Pgo8L2h0bWw+Cg==
--frontier--";

#[test]
fn test() {
    let mut s = stream::once(Bytes::from(test_str));
    let mut s = multipart_decoder::decode("frontier", s);
    let res = s.wait().unwrap();
    println!("{:?}", res);
}
