pub trait Allocator {
	unsafe fn alloc<T>(&mut self, value: T) -> &'static mut T;
}

#[derive(Debug)]
#[repr(align(8))]
pub struct Arena<const N: usize> {
	buffer: [u8; N],
	offset: usize,
}

impl<const N: usize> Arena<N> {
	pub const fn new() -> Self {
		Self {
			buffer: [0; N],
			offset: 0,
		}
	}
}

impl<const N: usize> Allocator for Arena<N> {
	unsafe fn alloc<T>(&mut self, value: T) -> &'static mut T {
		let size = core::mem::size_of::<T>();
		let align = core::mem::align_of::<T>();

		let aligned_offset = (self.offset + align - 1) & !(align - 1);
		if aligned_offset + size > N {
			panic!("Out of UI Arena Memory!");
		}

		unsafe {
			let ptr = self.buffer.as_mut_ptr().add(aligned_offset) as *mut T;
			core::ptr::write(ptr, value);
			self.offset = aligned_offset + size;
			&mut *ptr
		}
	}
}
