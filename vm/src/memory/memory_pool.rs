#![feature(array_ptr_get)]
use std::alloc::Layout;
use std::ops::Index;
use crate::objects::object::{ ObjectHeader, ObjectPointer, Pointer, ValidObject };

struct MemBlock<T> {
    max_elements: usize,
    element_size: usize,
    allocations: usize,
    elements: Vec<u8>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ValidObject> MemBlock<T> {
    fn new(max_elements: usize) -> Self {
        let element_size = Layout::new::<T>().size();
        let total_size = max_elements * element_size;
        MemBlock {
            max_elements,
            element_size,
            allocations: 0,
            elements: vec![0; total_size],
            _marker: std::marker::PhantomData,
        }
    }

    fn init(&mut self, index: usize, free_list_head: ObjectPointer) -> ObjectPointer {
        let mut current_head: ObjectPointer = free_list_head;

        for i in 0..self.max_elements {
            let o_pointer = unsafe {
                self.elements.as_ptr().add(i * self.element_size) as *mut ObjectPointer
            };

            unsafe {
                *o_pointer = current_head;
            }
            current_head = ObjectPointer::new_from_index_and_offset(index, i)
        }

        current_head
    }

    fn emplace(&mut self, offset: usize, value: T) -> ObjectPointer {
        self.allocations += 1;
        unsafe {
            let next_free = (self.elements.as_ptr().add(offset * self.element_size) as *const ObjectPointer)
                .read();

            let element_ptr = self.elements.as_ptr().add(offset * self.element_size) as *mut T;
            element_ptr.write(value);

            return next_free;
        }
    }

    fn drop(&mut self, ptr: ObjectPointer, next_free: ObjectPointer) -> ObjectPointer {
        if self.allocations < 1 {
            panic!("Attempting to deallocate from an empty block");
        }

        self.allocations -= 1;
        let o_pointer = unsafe {
            self.elements.as_ptr().add(ptr.offset() as usize * self.element_size)
        };

        unsafe {
            (o_pointer as *mut T).drop_in_place();
            (o_pointer as *mut ObjectHeader).write(ObjectHeader::null());
            (o_pointer as *mut ObjectPointer).write(next_free);
        }

        ptr
    }

    fn get(&self, offset: usize) -> Option<&T> {
        if offset >= self.max_elements {
            return None;
        }

        Some(&self[offset])
    }
}

impl<T> Index<usize> for MemBlock<T> {
    type Output = T;

    fn index(&self, offset: usize) -> &Self::Output {
        if offset >= self.max_elements {
            panic!("Offset out of bounds: {}", offset);
        }

        unsafe {
            let element_ptr = self.elements.as_ptr().add(offset * self.element_size) as *const T;
            &*element_ptr
        }
    }
}

pub trait MemAlloc {
    type Item;

    fn allocate(&mut self, value: Self::Item) -> Option<ObjectPointer>;
    fn deallocate(&mut self, ptr: ObjectPointer) -> Result<(), String>;
    fn to_type(&self, ptr: ObjectPointer) -> Option<&Self::Item>;
}

struct MemPool<T> {
    max_elements_per_block: usize,
    free_list: ObjectPointer,
    blocks: Vec<MemBlock<T>>,
}

impl<T> MemPool<T>
    where T: ValidObject
{
    pub fn new(max_elements_per_block: usize) -> Self {
        MemPool::<T> {
            max_elements_per_block,
            free_list: ObjectPointer::null(),
            blocks: vec![],
        }
    }

    pub fn initialized(max_elements_per_block: usize) -> Self {
        let mut pool = Self::new(max_elements_per_block);
        pool.add_block();
        pool
    }

    fn add_block(&mut self) {
        let new_index = self.blocks.len();
        let mut block = MemBlock::new(self.max_elements_per_block);
        self.free_list = block.init(new_index, self.free_list);
        self.blocks.push(block);
    }
}

impl<T> MemAlloc for MemPool<T>
    where T: ValidObject
{
    type Item = T;

    fn allocate(&mut self, value: T) -> Option<ObjectPointer> {
        if self.free_list.is_null() {
            self.add_block();
        }

        let target = self.free_list;
        let block = &mut self.blocks[target.block_index()];
        self.free_list = block.emplace(target.offset(), value);

        Some(target)
    }

    fn deallocate(&mut self, ptr: ObjectPointer) -> Result<(), String> {
        let block_index = ptr.block_index();
        if block_index >= self.blocks.len() {
            return Err(format!("Invalid block index: {}", block_index));
        }

        let block = &mut self.blocks[block_index];
        let offset = ptr.offset();
        let obj_ref = block.get(offset)
            .ok_or("Invalid offset".to_string())?;
        if T::is_valid(obj_ref) {
            self.free_list = block.drop(ptr, self.free_list);

            Ok(())
        } else {
            Err(String::from("Not a pointer to a valid object"))
        }
    }

    fn to_type(&self, _ptr: ObjectPointer) -> Option<&Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{
        number::Integer,
        symbol::Symbol,
    };

    static INITIALIZED_INTEGER_POOL: &[u8] = &[
        255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    static SECOND_INTEGER_POOL: &[u8] = &[
        9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        4, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        6, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        7, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        8, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    fn integer_to_bytes(i: Integer) -> Vec<u8> {
        let integer_size = Layout::new::<Integer>().size();
        unsafe {
            std::slice::from_raw_parts(
                (&i as *const Integer) as *const u8,
                integer_size,
            ).to_vec()
        }
    }

    fn initialize_block<T: ValidObject>(max_elements: usize) -> (MemBlock<T>, ObjectPointer) {
        let mut block = MemBlock::new(max_elements);
        let free_block = block.init(0, ObjectPointer::null());
        (block, free_block)
    }

    #[test]
    fn test_mem_block_initialization() {
        let (block, ptr) = initialize_block::<Integer>(10);

        assert_eq!(ptr, ObjectPointer::new_from_index_and_offset(0, 9));
        assert_eq!(block.elements, INITIALIZED_INTEGER_POOL);
    }

    #[test]
    fn test_mem_block_emplace_integer() {
        let (mut block, free_list_head) = initialize_block::<Integer>(10);

        let integer_size = Layout::new::<Integer>().size();
        let raw_offset = (free_list_head.offset() as usize) * integer_size;
        let next_free = block.emplace(free_list_head.offset(), Integer::new(42));

        assert_eq!(next_free, ObjectPointer::new_from_index_and_offset(0, 8));
        let initialized_integer = unsafe { 
            let raw_ptr = block.elements.as_ptr().add(raw_offset);
            std::slice::from_raw_parts(raw_ptr, integer_size)
        };

        let i42 = integer_to_bytes(Integer::new(42));

        assert_eq!(initialized_integer, &i42);
    }

    #[test]
    fn test_mem_block_deallocate_integer() {
        let (mut block, mut free_list_head) = initialize_block::<Integer>(10);

        free_list_head = block.emplace(free_list_head.offset(), Integer::new(42));
        let to_delete = free_list_head;
        free_list_head = block.emplace(free_list_head.offset(), Integer::new(256));
        free_list_head = block.emplace(free_list_head.offset(), Integer::new(16776960));

        let int_val = &block[to_delete.offset()];
        assert_eq!(int_val, &Integer::new(256));

        let next_free = block.drop(to_delete, free_list_head);
        assert_eq!(next_free, to_delete);
        assert!(!Integer::is_valid(&block[to_delete.offset()]));
    }

    #[test]
    fn test_mem_block_emplace_symbol() {
        let (mut block, free_list_head) = initialize_block::<Symbol>(10);

        block.emplace(free_list_head.offset(), Symbol::new("test_symbol".to_string()));

        assert_eq!(block[free_list_head.offset()], Symbol::new("test_symbol".to_string()));
    }

    #[test]
    fn test_mem_pool_initialization() {
        let mut pool: MemPool<Integer> = MemPool::new(10);
        pool.add_block();
        assert_eq!(pool.free_list, ObjectPointer::new_from_index_and_offset(0, 9));
        assert_eq!(pool.blocks[0].elements, INITIALIZED_INTEGER_POOL);
        pool.add_block();
        assert_eq!(pool.free_list, ObjectPointer::new_from_index_and_offset(1, 9));
        assert_eq!(pool.blocks[1].elements, SECOND_INTEGER_POOL);
    }

    #[test]
    fn test_mem_pool_integer_allocation() {
        let mut pool: MemPool<Integer> = MemPool::new(10);
        pool.allocate(Integer::new(42));

        let mut mutated_block = INITIALIZED_INTEGER_POOL.to_vec();
        let integer_size = Layout::new::<Integer>().size();
        mutated_block[integer_size * 9..integer_size * 10]
            .copy_from_slice(&integer_to_bytes(Integer::new(42)));

        assert_eq!(pool.free_list, ObjectPointer::new_from_index_and_offset(0, 8));
        assert_eq!(pool.blocks[0].elements, mutated_block);

    }

    #[test]
    fn test_mem_pool_multiple_integer_allocation() {
        let mut pool: MemPool<Integer> = MemPool::new(10);
        for i in 0i32..10 {
            pool.allocate(Integer::new(i));
        }


        assert_eq!(pool.free_list, ObjectPointer::null());

        pool.allocate(Integer::new(10));
        assert_eq!(pool.blocks.len(), 2);
        assert_eq!(pool.free_list, ObjectPointer::new_from_index_and_offset(1, 8));
    }
}
