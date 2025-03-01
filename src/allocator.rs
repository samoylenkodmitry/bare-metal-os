use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn init_allocator(heap_start: usize, heap_size: usize) {
    ALLOCATOR.lock().init(heap_start, heap_size);
}