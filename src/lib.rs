use core::ffi::c_void;
use std::sync::{Mutex, OnceLock};     
pub mod shared;

static IO_TARGET: OnceLock<IoTarget> = OnceLock::new();

static REQ: Slot<Request> = Slot::new();

pub fn evt_io_read(request: Request) {
    if let Some(target) = IO_TARGET.get() {
        request.send(target);
    }

    // request.complete(NtStatus::Success);

    REQ.set(request);

    //queue.enqueue(request);
    // let req = request.mark_cancellable();
}

pub fn cancel(request_id: &RequestId) {
    // Cancel sent request here

    try_complete(request_id, NtStatus::Cancelled);
}


pub fn on_completed(request_id: &RequestId) {
    try_complete(request_id, NtStatus::Success);
}

fn try_complete(request_id: &RequestId, status: NtStatus) {
    let req = REQ.take();
    if let Some(req) = req {
        if req.id() == *request_id  {
            req.complete(status);
        } else {
            REQ.set(req);
        }
    }
}


pub enum NtStatus {
    Success,
    Cancelled,
    Unknown
}

pub struct CancellationError<T>(NtStatus, T);

pub struct Request(WDFREQUEST);

impl Request {
    pub fn id(&self) -> RequestId {
        RequestId(self.0)
    }

    pub fn complete(self, _status: NtStatus) {
        // Call WdfRequestcomplete
    }

    pub fn mark_cancellable(self) -> Result<(), CancellationError<Self>> {
        let failed = false; // TODO: Call WdfRequestMarkCancelableEx
        if failed {
            Err(CancellationError(NtStatus::Unknown, self))
        } else {
            Ok(())
        }
    }

    pub fn send(&self, _target: &IoTarget) {
        // Call WdfRequestSend with self.0 and target.inner()
    }
}

pub struct RequestId(WDFREQUEST);

impl RequestId {
    fn as_ptr(&self) -> WDFREQUEST {
        self.0
    }
}

impl PartialEq for RequestId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RequestId {}


pub struct IoTarget(WDFIOTARGET);


impl IoTarget {
    pub fn inner(&self) -> WDFIOTARGET {
        self.0
    }
}

struct IoQueue;

impl IoQueue {
    pub fn queue(_request: Request) {
    }
}

pub struct Slot<T> {
    inner: Mutex<Option<T>>
}

impl<T> Slot<T> {
    pub const fn new() -> Self {
        Slot {
            inner: Mutex::new(None)
        }
    }

    pub fn take(&self) -> Option<T> {
        self.inner.lock().unwrap().take()
    }

    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = Some(value);
    }
}

// These maybe disastrously wrong
// Implemented just to make the code compile for now
unsafe impl Send for Request {}
unsafe impl Send for IoTarget {}
unsafe impl Sync for IoTarget {}


type WDFREQUEST = *mut c_void;
type WDFIOTARGET = *mut c_void;
