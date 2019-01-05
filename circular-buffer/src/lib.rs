#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

pub struct CircularBuffer<T> {
    stored: usize,
    capacity: usize,
    next_read: usize,
    next_write: usize,
    elements: Vec<Option<T>>,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            next_read: 0,
            next_write: 0,
            stored: 0,
            capacity,
            elements: vec![None; capacity],
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.capacity <= self.stored {
            Err(Error::FullBuffer)
        } else {
            let _ = std::mem::replace(&mut self.elements[self.next_write], Some(element));
            self.next_write = (self.next_write + 1) % self.capacity;
            self.stored += 1;

            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.stored <= 0 {
            Err(Error::EmptyBuffer)
        } else {
            let element = self.elements[self.next_read].take().unwrap();
            self.next_read = (self.next_read + 1) % self.capacity;

            self.stored -= 1;

            Ok(element)
        }
    }

    pub fn clear(&mut self) {
        self.stored = 0;
        self.next_write = 0;
        self.next_read = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if self.stored < self.capacity {
            let _ = self.write(element);
        } else {
            let _ = std::mem::replace(&mut self.elements[self.next_write], Some(element));

            self.next_write = (self.next_write + 1) % self.capacity;
            self.next_read = (self.next_read + 1) % self.capacity;
        }
    }
}
