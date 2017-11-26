// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0 or the MIT license
// http://opensource.org/licenses/MIT, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// A wrapper for Option<T>, where None > Some(_).
// No other function.
use core::cmp::Ordering::{self, Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
// None > Some, always
pub struct RevOption<T>(pub Option<T>);

impl<T: PartialOrd> PartialOrd<RevOption<T>> for RevOption<T> {
	#[inline]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self.0.is_none(), other.0.is_none()) {
			(true, false) => Some(Greater),
			(true, true)  => Some(Equal),
			(false, true) => Some(Less),
			_             => self.0.partial_cmp(&other.0),
		}
	}
}

impl<T: Ord> Ord for RevOption<T> {
	#[inline]
	fn cmp(&self, other: &Self) -> Ordering {
		match (self.0.is_none(), other.0.is_none()) {
			(true, false) => Greater,
			(true, true)  => Equal,
			(false, true) => Less,
			_             => self.0.cmp(&other.0),
		}
	}
}

#[cfg(test)]
mod test {
	use super::RevOption;
	#[test]
	fn rev_option() {
		assert!( RevOption(None)   >  RevOption(Some(2)));
		assert!( RevOption(None)   >= RevOption(Some(2)));
		assert!( !(RevOption(None) <  RevOption(Some(2))));
	}

	#[test]
	fn normal_option() {
		assert!( None   <  Some(2));
		assert!( None   <= Some(2));
		assert!(!( None >  Some(2) ) );
	}
}
