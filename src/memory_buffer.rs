

pub trait MemoryTypedBuffer<S>
{
    fn allocate(&mut self) -> S;
    fn deallocate(&mut self, free: S);

    fn clone(&mut self, source: &S) -> S;
}

pub struct SimpleNonbufferedAllocator
{}

impl<T: Default + Clone> MemoryTypedBuffer<T> for SimpleNonbufferedAllocator
{
    fn allocate(&mut self) -> T {
        T::default()
    }

    fn deallocate(&mut self, _free: T) {
    }

    fn clone(&mut self, source: &T) -> T {
        source.clone()
    }
}

pub struct FixedSizeVectorsBuffer<T: Default + Clone>
{
    buffer: Vec<Vec<T>>,
    fixed_vector_size: usize
}

impl<T: Default + Clone> FixedSizeVectorsBuffer<T>
{
    fn new(init_buffer_size: usize, fixed_vectors_size: usize) -> Self
    {
        FixedSizeVectorsBuffer {
            buffer: Vec::with_capacity(init_buffer_size),
            fixed_vector_size: fixed_vectors_size
        }
    }
}

impl<T: Default + Clone> MemoryTypedBuffer<Vec<T>> for FixedSizeVectorsBuffer<T>
{
    fn allocate(&mut self) -> Vec<T> {
        let val = self.buffer.pop();

        if let Some(val) = val
        {
            val
        }
        else
        {
            vec![T::default(); self.fixed_vector_size]
        }
    }

    fn deallocate(&mut self, free: Vec<T>) {
        self.buffer.push(free)
    }

    fn clone(&mut self, source: &Vec<T>) -> Vec<T> {
        let mut new = self.allocate();

        source.clone_into(&mut new);

        new
    }
}
