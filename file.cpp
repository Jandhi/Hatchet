#include <iostream>

void print(const char* arg) {
    std::cout << arg << std::endl;
}

int main() {
    print("Hello world from c++!");
    return 0;
}

template <typename T> 
using List = std::vector<T>;

template <typename T>
void fn_list_get(List<T>* list, int index) {
    return *list[index]
}