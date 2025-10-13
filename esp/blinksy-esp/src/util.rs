use core::fmt::Debug;

use heapless::Vec;

/// Returns an iterator that yields heapless::Vec chunks from `iter`.
pub fn chunked<I, const BUFFER_SIZE: usize>(
    mut iter: I,
    chunk_size: usize,
) -> impl Iterator<Item = Vec<I::Item, BUFFER_SIZE>>
where
    I: Iterator,
    I::Item: Debug,
{
    core::iter::from_fn(move || {
        if chunk_size == 0 {
            return None;
        }

        let mut buf: Vec<I::Item, BUFFER_SIZE> = Vec::new();

        for _ in 0..chunk_size {
            match iter.next() {
                Some(item) => {
                    let _ = buf.push(item).expect("chunked: chunk size > buffer size");
                }
                None => break,
            }
        }

        if buf.is_empty() {
            None
        } else {
            Some(buf)
        }
    })
}
