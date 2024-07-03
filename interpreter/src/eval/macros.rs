#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__try_or_fail {
	($e:expr) => {
		match $e {
			Ok(v) => v,
			Err(e) => return $crate::etable::Control::Exit(e.into()),
		}
	};
}
#[doc(inline)]
pub use __eval__macros__try_or_fail as try_or_fail;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__pop {
	( $machine:expr, $( $x:ident ),* ) => (
		$(
			let $x = match $machine.stack.pop() {
				Ok(value) => value,
				Err(e) => return $crate::etable::Control::Exit(e.into()),
			};
		)*
	);
}
#[doc(inline)]
pub use __eval__macros__pop as pop;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__pop_u256 {
	( $machine:expr, $( $x:ident ),* ) => (
		$(
			let $x = match $machine.stack.pop() {
				Ok(value) => U256::from_big_endian(&value[..]),
				Err(e) => return $crate::etable::Control::Exit(e.into()),
			};
		)*
	);
}
#[doc(inline)]
pub use __eval__macros__pop_u256 as pop_u256;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__push {
	( $machine:expr, $( $x:expr ),* ) => (
		$(
			match $machine.stack.push($x) {
				Ok(()) => (),
				Err(e) => return $crate::etable::Control::Exit(e.into()),
			}
		)*
	)
}
#[doc(inline)]
pub use __eval__macros__push as push;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__push_u256 {
	( $machine:expr, $( $x:expr ),* ) => (
		$(
			let mut value = H256::default();
			$x.to_big_endian(&mut value[..]);
			match $machine.stack.push(value) {
				Ok(()) => (),
				Err(e) => return $crate::etable::Control::Exit(e.into()),
			}
		)*
	)
}
#[doc(inline)]
pub use __eval__macros__push_u256 as push_u256;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op1_u256_fn {
	($machine:expr, $op:path) => {{
		$crate::eval::macros::pop_u256!($machine, op1);
		let ret = $op(op1);
		$crate::eval::macros::push_u256!($machine, ret);

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op1_u256_fn as op1_u256_fn;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op2_u256_bool_ref {
	($machine:expr, $op:ident) => {{
		$crate::eval::macros::pop_u256!($machine, op1, op2);
		let ret = op1.$op(&op2);
		$crate::eval::macros::push_u256!($machine, if ret { U256::one() } else { U256::zero() });

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op2_u256_bool_ref as op2_u256_bool_ref;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op2_u256 {
	($machine:expr, $op:ident) => {{
		$crate::eval::macros::pop_u256!($machine, op1, op2);
		let ret = op1.$op(op2);
		$crate::eval::macros::push_u256!($machine, ret);

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op2_u256 as op2_u256;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op2_u256_tuple {
	($machine:expr, $op:ident) => {{
		$crate::eval::macros::pop_u256!($machine, op1, op2);
		let (ret, ..) = op1.$op(op2);
		$crate::eval::macros::push_u256!($machine, ret);

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op2_u256_tuple as op2_u256_tuple;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op2_u256_fn {
	($machine:expr, $op:path) => {{
		$crate::eval::macros::pop_u256!($machine, op1, op2);
		let ret = $op(op1, op2);
		$crate::eval::macros::push_u256!($machine, ret);

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op2_u256_fn as op2_u256_fn;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__op3_u256_fn {
	($machine:expr, $op:path) => {{
		$crate::eval::macros::pop_u256!($machine, op1, op2, op3);
		let ret = $op(op1, op2, op3);
		$crate::eval::macros::push_u256!($machine, ret);

		$crate::etable::Control::Continue
	}};
}
#[doc(inline)]
pub use __eval__macros__op3_u256_fn as op3_u256_fn;

#[doc(hidden)]
#[macro_export]
macro_rules! __eval__macros__as_usize_or_fail {
	($v:expr) => {{
		if $v > U256::from(usize::MAX) {
			return $crate::etable::Control::Exit(ExitFatal::NotSupported.into());
		}

		$v.as_usize()
	}};

	($v:expr, $reason:expr) => {{
		if $v > U256::from(usize::MAX) {
			return $crate::etable::Control::Exit($reason.into());
		}

		$v.as_usize()
	}};
}
#[doc(inline)]
pub use __eval__macros__as_usize_or_fail as as_usize_or_fail;
