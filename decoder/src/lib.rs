// #[macro_use]
// extern crate nom;
extern crate mime;
extern crate bytes;
extern crate futures;
extern crate hyper;

use bytes::{Bytes, BytesMut};
use futures::{Stream, Poll, Async};
use hyper::{Headers};

// use nom::{IResult};

// named!(<i64>,
//
// );

pub enum Frame {
    Headers(pub Headers),
    Body(pub Bytes),
}

enum State {
    IgnoreFirst,
    Headers(Headers, BytesMut),
    Body,
}

pub struct Decoder<S> {
    stream: S,
    state: State
}

impl<S> Stream for Decoder<S>
    where S: Stream<Item=Bytes>,
{
    type Item = Frame;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<Frame>, S::Error> {

    }
}

pub fn decode<S: Stream<Item=Bytes>>(boundary: &[u8], input: S) -> Decoder<S> {
    Decoder {
        stream: input,
        state: State:IgnoreFirst,
    }
}
