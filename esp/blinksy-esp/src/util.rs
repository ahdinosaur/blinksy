use heapless::Vec;

/// Returns an iterator that yields heapless::Vec chunks from `iter`.
pub(crate) fn chunked<I, const CHUNK_SIZE: usize>(
    mut iter: I,
) -> impl Iterator<Item = Vec<I::Item, CHUNK_SIZE>>
where
    I: Iterator,
{
    core::iter::from_fn(move || {
        if CHUNK_SIZE == 0 {
            return None;
        }

        let mut buf: Vec<I::Item, CHUNK_SIZE> = Vec::new();

        for _ in 0..CHUNK_SIZE {
            match iter.next() {
                Some(item) => {
                    // Guaranteed to fit because we push at most CHUNK_SIZE items.
                    let _ = buf.push(item);
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
