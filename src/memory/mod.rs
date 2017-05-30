pub use self::area_frame_allocater::AreaFrameAllocater;

mod area_frame_allocater;

pub const PAGE_SIZE: usize = 4096;

mod paging;

use self::paging::PhysicalAddress;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAllocater {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
