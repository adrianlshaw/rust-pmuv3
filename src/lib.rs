#![feature(asm)]
#[macro_use]
extern crate bitflags;

bitflags! (
	pub flags EventFilterFlags: usize {
		const EXCLUDE_EL1 = 1 << 31,
		const EXCLUDE_EL0 = 1 << 30,
		const EXCLUDE_EL2 = 1 << 27
	}
);

bitflags! (
	pub flags PmcrFlags: usize {
		const PMCR_ENABLE	= 1 << 0,
		const PMCR_PERF_RESET	= 1 << 1,
		const PMCR_CNT_RESET	= 1 << 2
	}
);

const PMCR_N_SHIFT		: u32 = 11;
const PMCR_N_MASK		: u32 = 0x1f;
const EVENT_TYPE_MASK		: u32 = 0xc80003ff;
const EVTYPE_EVENT		: u32 = 0x3ff;
const IDX_CYCLE_COUNTER 	: u32 = 0;
const IDX_COUNTER0 		: u32 = 1;
const OVSR_MASK 		: u32 = 0xffffffff;
const OVERFLOWED_MASK		: u32 = OVSR_MASK;
const MAX_COUNTERS 		: u32 = 32;
const COUNTER_MASK		: u32 = (MAX_COUNTERS - 1);

fn idx_to_counter(x: u32) -> u32 {
	let mut ret;
	ret = (x - IDX_COUNTER0) & COUNTER_MASK;
	return ret
}

fn overflowed(pmovsr: u32) -> u32 {
	return pmovsr & OVERFLOWED_MASK;
}

#[cfg(any(target_arch = "aarch64"))]
fn private_isb() {
	unsafe {
		asm!("isb sy" : : : "memory");
	}
}


#[cfg(any(target_arch = "aarch64"))]
pub fn get_pmu_reset_flags() {
	let mut value: u32;
	unsafe {
		asm!("mrs %0, pmovsclr_el0" : "=r" (value));
		value &= OVSR_MASK;
		asm!("msr pmovsclr_el0, %0" :: "r" (value));
	}
	return value;
}


#[cfg(any(target_arch = "aarch64"))]
fn select_counter(idx: u32) {
	let mut counter: u32;
	unsafe {
		asm!("msr pmselr_el0, %0" :: "r" (counter));
		private_isb();
	}
}


#[cfg(any(target_arch = "aarch64"))]
fn write_counter(idx: u32, value: u32) {
	if (idx == IDX_CYCLE_COUNTER) {
		unsafe {
			asm!("msr pmccntr_el0, %0" :: "r" (value));
		}
	}
	else {
		select_counter(idx);
		unsafe {
			asm!("msr pmxevcntr_el0, %0" :: "r" (value));
		}
	}
}


#[cfg(any(target_arch = "aarch64"))]
fn write_evtype(idx: u32, val: u32) {
	select_counter(idx);
	val &= EVENT_TYPE_MASK as u32;
	unsafe {
		asm!("msr pmxevtyper_el0, %0" :: "r" (val));
	}
}


#[test]
fn it_works() {


}