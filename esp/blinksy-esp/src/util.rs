use core::fmt::Debug;

use heapless::Vec;

pub fn for_each_chunk<const BUFFER_SIZE: usize, Iter, Error>(
    mut iter: Iter,
    chunk_size: usize,
    mut f: impl FnMut(&mut Vec<Iter::Item, BUFFER_SIZE>) -> Result<(), Error>,
) -> Result<(), Error>
where
    Iter: Iterator,
    Iter::Item: Debug,
{
    if chunk_size == 0 {
        return Ok(());
    }

    let mut buffer: Vec<Iter::Item, BUFFER_SIZE> = Vec::new();

    loop {
        buffer.clear();

        for _ in 0..chunk_size {
            match iter.next() {
                Some(item) => {
                    // Assumes chunk_size <= BUFFER_SIZE
                    buffer.push(item).expect("chunk size > buffer size");
                }
                None => break,
            }
        }

        if buffer.is_empty() {
            break;
        }

        f(&mut buffer)?;
    }

    Ok(())
}

#[cfg(feature = "async")]
pub async fn for_each_chunk_async<const BUFFER_SIZE: usize, Iter, Error>(
    mut iter: Iter,
    chunk_size: usize,
    mut f: impl AsyncFnMut(&mut Vec<Iter::Item, BUFFER_SIZE>) -> Result<(), Error>,
) -> Result<(), Error>
where
    Iter: Iterator,
    Iter::Item: Debug,
{
    if chunk_size == 0 {
        return Ok(());
    }

    let mut buffer: Vec<Iter::Item, BUFFER_SIZE> = Vec::new();

    loop {
        buffer.clear();

        for _ in 0..chunk_size {
            match iter.next() {
                Some(item) => {
                    // Assumes chunk_size <= BUFFER_SIZE
                    buffer.push(item).expect("chunk size > buffer size");
                }
                None => break,
            }
        }

        if buffer.is_empty() {
            break;
        }

        f(&mut buffer).await?;
    }

    Ok(())
}
