use crate::hook_info::EVENT_FLAGS;
use winapi::um::{
    handleapi::CloseHandle,
    synchapi::{CreateEventA, OpenEventA, SetEvent, WaitForSingleObject},
    winbase::WAIT_OBJECT_0,
};

pub struct Event {
    handle: usize,
}

impl Event {
    pub fn create(name: Option<&str>) -> Option<Self> {
        let name = if let Some(name) = name {
            format!("{}\0", name).as_ptr() as _
        } else {
            std::ptr::null_mut()
        };

        let event = unsafe { CreateEventA(std::ptr::null_mut(), false as _, false as _, name) };
        if event.is_null() {
            None
        } else {
            log::trace!("Created the event {:?} = 0x{:x}", name, event as usize);

            Some(Self { handle: event as usize })
        }
    }

    pub fn open<S: AsRef<str>>(name: S) -> Option<Self> {
        let event = unsafe { OpenEventA(EVENT_FLAGS, false as _, format!("{}\0", name.as_ref()).as_ptr() as _) };

        if event.is_null() {
            None
        } else {
            log::trace!("Created the event {:?} = 0x{:x}", name.as_ref(), event as usize);
            Some(Self { handle: event as usize })
        }
    }

    /// Sets the event to the signalled state.
    pub fn signal(&self) -> Option<()> {
        if unsafe { SetEvent(self.handle as _) } == 0 {
            None
        } else {
            Some(())
        }
    }

    /// Checks whether the event is signalled.
    pub fn signalled(&self) -> Option<()> {
        match unsafe { WaitForSingleObject(self.handle as _, 0) } {
            WAIT_OBJECT_0 => Some(()),
            _ => None,
        }
    }

    /// Returns the internal handle
    pub fn handle(&self) -> usize {
        self.handle
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        log::trace!("Dropping the event");
        unsafe { CloseHandle(self.handle as _) };
    }
}
