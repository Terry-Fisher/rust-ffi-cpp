


use std::ffi::c_void;
use std::ptr;

// Функция get возвращает указатель на данные, которые должны быть освобождены в Rust
#[no_mangle]
pub extern "C" fn get(out_len: *mut usize) -> *mut u8 {
    // Создаем вектор данных
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];

    // Сохраняем длину данных в указатель `out_len`
    unsafe {
        if !out_len.is_null() {
            *out_len = data.len();
        }
    }

    // Преобразуем вектор в указатель на данные
    let ptr = data.as_mut_ptr();

    // Предотвращаем освобождение вектора Rust
    std::mem::forget(data);

    // Возвращаем указатель на данные
    ptr
}

// Функция освобождает память, выделенную в Rust
#[no_mangle]
pub extern "C" fn free_rust_memory(data_ptr: *mut u8, len: usize) {
    if !data_ptr.is_null() {
        // Преобразуем указатель обратно в вектор и освобождаем память
        unsafe {
            println!("Drop");
            let _ = Vec::from_raw_parts(data_ptr, len, len);
        }
    }
}

// Функция set принимает данные и длину из C++
#[no_mangle]
pub extern "C" fn set(data_ptr: *const u8, len: usize) {
    if !data_ptr.is_null() {
        // Создаем срез на основе указателя и копируем данные в вектор
        let slice = unsafe { std::slice::from_raw_parts(data_ptr, len) };
        let data = slice.to_vec();

        // Используем полученные данные
        println!("Received data: {:?}", data);
    }
}




pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
