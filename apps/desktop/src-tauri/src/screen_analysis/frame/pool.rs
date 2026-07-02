use std::sync::{Arc, Mutex};
use crate::screen_analysis::frame::scaler::RawFrameBuffer;

/// A reusable buffer leased from the FramePool.
/// When dropped, it returns the buffer to the pool.
pub struct FrameLease {
    pub buffer: RawFrameBuffer,
    pool: Arc<Mutex<Vec<RawFrameBuffer>>>,
}

impl FrameLease {
    pub fn new(buffer: RawFrameBuffer, pool: Arc<Mutex<Vec<RawFrameBuffer>>>) -> Self {
        Self { buffer, pool }
    }
}

impl Drop for FrameLease {
    fn drop(&mut self) {
        // Return the empty/reusable buffer to the pool.
        // We take ownership of the inner buffer by replacing it with a dummy,
        // since we only need the capacity, not the data.
        let capacity = self.buffer.pixels.capacity();
        let mut empty_vec = Vec::with_capacity(capacity);
        std::mem::swap(&mut self.buffer.pixels, &mut empty_vec);
        
        let mut pool = self.pool.lock().unwrap();
        // Create a new RawFrameBuffer that reuses the allocated capacity
        pool.push(RawFrameBuffer::new(empty_vec, self.buffer.width, self.buffer.height));
    }
}

/// FramePool manages a ring of pre-allocated frame buffers.
/// Ensures that Desktop Duplication does not allocate memory on every frame.
pub struct FramePool {
    available: Arc<Mutex<Vec<RawFrameBuffer>>>,
}

impl FramePool {
    pub fn new(initial_capacity: usize, width: u32, height: u32) -> Self {
        let mut buffers = Vec::with_capacity(initial_capacity);
        let pixel_count = (width * height * 4) as usize;
        
        for _ in 0..initial_capacity {
            let pixels = Vec::with_capacity(pixel_count); // Pre-allocate
            buffers.push(RawFrameBuffer::new(pixels, width, height));
        }

        Self {
            available: Arc::new(Mutex::new(buffers)),
        }
    }

    /// Acquires a FrameLease from the pool.
    pub fn acquire(&self, width: u32, height: u32) -> FrameLease {
        let mut pool = self.available.lock().unwrap();
        if let Some(mut buffer) = pool.pop() {
            buffer.width = width;
            buffer.height = height;
            FrameLease::new(buffer, self.available.clone())
        } else {
            // Pool exhausted, allocate a new one (should be rare if sized correctly)
            log::warn!("FramePool exhausted, allocating new frame buffer");
            let pixel_count = (width * height * 4) as usize;
            let pixels = Vec::with_capacity(pixel_count);
            FrameLease::new(RawFrameBuffer::new(pixels, width, height), self.available.clone())
        }
    }
}
