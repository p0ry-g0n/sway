script;

fn main() -> u64 { 
	// return value of an asm block is the popped stack?
	let a: u64 = miden_asm {
		push.40.2;
		u32wrapping_add;
	};
	// we should know that we only need to pop one stack element because the type is u64
	a
	
}
