#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	pub trait MyRuntimeApi {
		fn call_extrinsic(
			something: u32
		) -> Result<(), ()>;
	}
}