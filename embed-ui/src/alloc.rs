pub trait Allocator<'m> {
	fn alloc<'s, T>(&'s mut self, value: T) -> &'m mut T;
}

#[derive(Debug)]
pub struct Arena<const N: usize> {
	pub buffer: [u8; N],
	pub offset: usize,
}

impl<const N: usize> Arena<N> {
	pub const fn new() -> Self {
		Self {
			buffer: [0; N],
			offset: 0,
		}
	}
}

impl<'m, const N: usize> Allocator<'m> for Arena<N> {
	fn alloc<'s, T>(&'s mut self, value: T) -> &'m mut T {
		let size = core::mem::size_of::<T>();
		let align = core::mem::align_of::<T>();

		let aligned_offset = (self.offset + align - 1) & !(align - 1);
		if aligned_offset + size > N {
			panic!("Arena out of memory");
		}

		unsafe {
			let ptr = self.buffer.as_mut_ptr().add(aligned_offset) as *mut T;
			core::ptr::write(ptr, value);
			self.offset = aligned_offset + size;
			&mut *ptr
		}
	}
}
