use bevy::prelude::warn;
use bytes::BytesMut;

const INITIAL_BUFFER_CAPACITY: usize = 2048;
const MINIMUM_BUFFER_CAPACITY: usize = INITIAL_BUFFER_CAPACITY / 4;
const CRITICAL_BUFFER_CAPACITY: usize = MINIMUM_BUFFER_CAPACITY / 2;

pub fn create_buffer() -> BytesMut {
    BytesMut::with_capacity(INITIAL_BUFFER_CAPACITY)
}

/// Temporary (TM) quick and easy solution to make sure we don't have to figure out how to handle incoming messages larger than the local buffer capacity for the time being.
/// Should be called after a split() to ensure buffer.len() is 0.
pub fn reclaim_buffer_capacity_if_necessary(buffer: &mut BytesMut) {
    if buffer.capacity() < MINIMUM_BUFFER_CAPACITY {
        if buffer.capacity() < CRITICAL_BUFFER_CAPACITY {
            // If this ever gets triggered, it's time to rewrite how these buffers work or to increase INITIAL_BUFFER_SIZE.
            warn!(
                "Buffer capacity was below critical threshold: {} ",
                buffer.capacity()
            )
        }

        buffer.reserve(INITIAL_BUFFER_CAPACITY);
    }
}
