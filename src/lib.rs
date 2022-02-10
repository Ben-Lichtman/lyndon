pub struct Lyndon<const N: usize, const K: u8> {
	len: usize,
	buffer: [u8; N],
}

impl<const N: usize, const K: u8> Lyndon<N, K> {
	pub fn new() -> Self {
		Self {
			len: 0,
			buffer: [0; N],
		}
	}
}

impl<const N: usize, const K: u8> Default for Lyndon<N, K> {
	fn default() -> Self { Self::new() }
}

impl<const N: usize, const K: u8> Lyndon<N, K> {
	pub fn iter(&mut self) -> LyndonIterator<N, K> { LyndonIterator { lyndon: self } }
}

pub struct LyndonIterator<'a, const N: usize, const K: u8> {
	lyndon: &'a mut Lyndon<N, K>,
}

// This can not be a true iterator until GAT is stabilised :(
impl<const N: usize, const K: u8> LyndonIterator<'_, N, K> {
	pub fn next_item(&mut self) -> Option<&[u8]> {
		let mut len = self.lyndon.len;
		if len == 0 {
			self.lyndon.buffer[0] = 0;
			self.lyndon.len = 1;
			return Some(&self.lyndon.buffer[..1]);
		}

		for to in len..N {
			self.lyndon.buffer[to] = self.lyndon.buffer[to % len];
		}
		len = N
			- (0..N)
				.rev()
				.take_while(|&i| self.lyndon.buffer[i] == K - 1)
				.count();

		self.lyndon.len = len;

		match len {
			0 => None,
			_ => {
				// SAFETY: we have already made sure that len is in-bounds
				unsafe { *self.lyndon.buffer.get_unchecked_mut(len - 1) += 1 };
				Some(&self.lyndon.buffer[..len])
			}
		}
	}
}
