//! An OpenCL event.

// use std::fmt::{std::fmt::Display, std::fmt::Formatter, Result as std::fmt::Result};
use std;
use std::convert::Into;
// use error::Result as OclResult;
// use cl_h::cl_event;
use core::{self, /*EventPtr,*/ Event as EventCore, EventInfo, EventInfoResult, ProfilingInfo, ProfilingInfoResult};
use standard;
// use util;


/// An event representing a command or user created event.
#[derive(Clone, Debug)]
pub struct Event(EventCore);

impl Event {
	/// Creates a new `Event` from a `EventCore`.
	///
	/// ### Safety 
	///
	/// Not meant to be called directly.
	pub fn new(event_core: EventCore) -> Event {
		Event(event_core)
	}

	// /// Creates a new null `Event`.
	// ///
	// /// ### Safety 
	// ///
	// /// Don't use unless you know what you're doing.
	// pub unsafe fn new_null() -> Event {
	// 	Event(EventCore::null())
	// }

	/// Returns info about the event. 
	pub fn info(&self, info_kind: EventInfo) -> EventInfoResult {
		match core::get_event_info(&self.0, info_kind) {
			Ok(pi) => pi,
			Err(err) => EventInfoResult::Error(Box::new(err)),
		}
	}

	/// Returns info about the event. 
	pub fn profiling_info(&self, info_kind: ProfilingInfo) -> ProfilingInfoResult {
		match core::get_event_profiling_info(&self.0, info_kind) {
			Ok(pi) => pi,
			Err(err) => ProfilingInfoResult::Error(Box::new(err)),
		}
	}

	/// Returns a string containing a formatted list of event properties.
	pub fn to_string(&self) -> String {
		self.clone().into()
	}

	/// Returns the underlying `EventCore`.
	pub fn core_as_ref(&self) -> &EventCore {
		&self.0
	}

	/// Returns the underlying `EventCore`.
	pub fn core_as_mut(&mut self) -> &mut EventCore {
		&mut self.0
	}
}

// unsafe impl EventPtr for Event {}
// unsafe impl EventPtr for Event {
// 	fn as_ptr(&self) -> cl_event {
// 		self.0.as_ptr()
// 	}
// }

impl Into<String> for Event {
	fn into(self) -> String {
		format!("{}", self)
	}
}

// impl Into<EventCore> for Event {
// 	fn into(self) -> EventCore {
// 		self.as_core()
// 	}
// }

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let (begin, delim, end) = if standard::INFO_FORMAT_MULTILINE {
    		("\n", "\n", "\n")
    	} else {
    		("{ ", ", ", " }")
		};

		write!(f, "[Event]: {b}\
				CommandQueue: {}{d}\
	            CommandType: {}{d}\
	            ReferenceCount: {}{d}\
	            CommandExecutionStatus: {}{d}\
	            Context: {}{e}\
			",
			core::get_event_info(&self.0, EventInfo::CommandQueue).unwrap(),
	        core::get_event_info(&self.0, EventInfo::CommandType).unwrap(),
	        core::get_event_info(&self.0, EventInfo::ReferenceCount).unwrap(),
	        core::get_event_info(&self.0, EventInfo::CommandExecutionStatus).unwrap(),
	        core::get_event_info(&self.0, EventInfo::Context).unwrap(),
	        b = begin,
			d = delim,
			e = end,
		)
    }
}


//     // ##################################################
//     // ##################### EVENT ######################
//     // ##################################################

//     // [FIXME]: Complete this section.
//     // pub enum EventInfo {
//     //     CommandQueue = cl_h::CL_EVENT_COMMAND_QUEUE as isize,
//     //     CommandType = cl_h::CL_EVENT_COMMAND_TYPE as isize,
//     //     ReferenceCount = cl_h::CL_EVENT_REFERENCE_COUNT as isize,
//     //     CommandExecutionStatus = cl_h::CL_EVENT_COMMAND_EXECUTION_STATUS as isize,
//     //     Context = cl_h::CL_EVENT_CONTEXT as isize,
//     // }

//     println!("EventInfo:\n\
// 			{t}CommandQueue: {}\n\
//             {t}CommandType: {}\n\
//             {t}ReferenceCount: {}\n\
//             {t}CommandExecutionStatus: {}\n\
//             {t}Context: {}\n\
// 		",
// 		core::get_event_info(event, EventInfo::CommandQueue).unwrap(),
//         core::get_event_info(event, EventInfo::CommandType).unwrap(),
//         core::get_event_info(event, EventInfo::ReferenceCount).unwrap(),
//         core::get_event_info(event, EventInfo::CommandExecutionStatus).unwrap(),
//         core::get_event_info(event, EventInfo::Context).unwrap(),
// 		t = util::TAB,
// 	);

//     // ##################################################
//     // ################ EVENT PROFILING #################
//     // ##################################################

//     // [FIXME]: Complete this section.
//     // pub enum ProfilingInfo {
//     //     Queued = cl_h::CL_PROFILING_COMMAND_QUEUED as isize,
//     //     Submit = cl_h::CL_PROFILING_COMMAND_SUBMIT as isize,
//     //     Start = cl_h::CL_PROFILING_COMMAND_START as isize,
//     //     End = cl_h::CL_PROFILING_COMMAND_END as isize,
//     // }

//     println!("ProfilingInfo:\n\
// 			{t}Queued: {}\n\
// 	    	{t}Submit: {}\n\
// 	    	{t}Start: {}\n\
// 	    	{t}End: {}\n\
// 		",
// 		core::get_event_profiling_info(event, ProfilingInfo::Queued).unwrap(),
//         core::get_event_profiling_info(event, ProfilingInfo::Submit).unwrap(),
//         core::get_event_profiling_info(event, ProfilingInfo::Start).unwrap(),
//         core::get_event_profiling_info(event, ProfilingInfo::End).unwrap(),
// 		t = util::TAB,
// 	);


//     print!("\n");
// }