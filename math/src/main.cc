#include <iostream>

extern "C" int32_t add_i32(int32_t a, int32_t b);
extern "C" int32_t sub_i32(int32_t a, int32_t b);
extern "C" int32_t mul_i32(int32_t a, int32_t b);

int main()
{
	std::cout << "add_i32(1, 2): " << add_i32(1, 2) << "\n";
	std::cout << "sub_i32(5, 2): " << sub_i32(5, 2) << "\n";
	std::cout << "mul_i32(6, 6): " << mul_i32(6, 6) << "\n";

	return 0;
}
