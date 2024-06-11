#[macro_export]
macro_rules! err_return {
    ($val:expr, $msg:expr, $ret_val:expr) => {
        match $val {
            Ok(x) => x,
            Err(e) => {
                println!($msg, e);
                return $ret_val;
            }
        }
    };
    ($val:expr, $msg:expr) => {
        match $val {
            Ok(x) => x,
            Err(e) => {
                println!($msg, e);
                return
            }
        }
    };
}

// Assuming compiler is optimizing this to a single operation
#[inline(always)]
pub fn divrem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

pub fn convert_slice<T,U>(v: &[T]) -> &[U] {
    // println!("convert slice {} -> {}, {} -> {}", std::any::type_name::<T>(), std::any::type_name::<T>(), std::mem::size_of::<U>(), std::mem::size_of::<U>());
    let size_from = std::mem::size_of::<T>();
    let size_to = std::mem::size_of::<U>();
    let len_from = v.len();
    let len_to = len_from * size_from / size_to;
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const U, len_to) }
}

// pub fn convert_slice_sized<T,U, const N: usize>>(v: &[T]) -> &[U; N] {
//     unsafe { std::slice::from_raw_parts(v.as_ptr() as *const U, v.len() * 4) }
// }
