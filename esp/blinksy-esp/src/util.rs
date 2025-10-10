use heapless::Vec;

/// Returns an iterator that yields heapless::Vec chunks from `iter`.
pub fn chunked<I, const BUFFER_SIZE: usize>(
    mut iter: I,
    chunk_size: usize,
) -> impl Iterator<Item = Vec<I::Item, BUFFER_SIZE>>
where
    I: Iterator,
{
    let size = core::cmp::min(chunk_size, BUFFER_SIZE);

    core::iter::from_fn(move || {
        if size == 0 {
            return None;
        }

        let mut buf: Vec<I::Item, BUFFER_SIZE> = Vec::new();

        for _ in 0..size {
            match iter.next() {
                Some(item) => {
                    // Safe because size <= BUFFER_SIZE, so we never exceed capacity.
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
