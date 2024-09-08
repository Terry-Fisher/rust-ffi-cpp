#include <iostream>
#include <dlfcn.h>  // Для загрузки динамической библиотеки
#include <vector>
#include <map>



using namespace std;



// Прототипы функций
extern "C" {
    uint8_t* get(size_t* out_len);
    void set(uint8_t* data, size_t len);
    void free_rust_memory(uint8_t* data, size_t len);
}

int main() {
    // Получаем данные из Rust
    size_t len = 0;
    uint8_t* raw_data = get(&len);

    // Создаем std::vector на основе указателя и длины
    std::vector<uint8_t> data(raw_data, raw_data + len);

    // Проверяем, что данные получены корректно
    if (raw_data != nullptr) {
        // Вывод данных из Rust
        std::cout << "Data from Rust: [";
        for (size_t i = 0; i < len; ++i) {
            if (i == (len-1) ) {
                std::cout << static_cast<int>(data[i]) << "]";
            } else {
                std::cout << static_cast<int>(data[i]) << ", ";
            }
        }
        std::cout << std::endl;

        // Освобождаем память, выделенную в Rust
        free_rust_memory(raw_data, len);
    }



    // Пример отправки данных в Rust через `set`
    std::vector<uint8_t> new_data = {10, 20, 30, 40};

    set(new_data.data(), new_data.size());





    return 0;
}
