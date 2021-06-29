#include <iostream>
#include <vector>

int main() {
	int ar[] = {3, 1, 4, 1, 5, 9};
	for (int &x : ar) {
		std::cout << x << std::endl;
	}

	std::vector<int> v {1, 2, 3, 4, 5, 6, 7};
	for (const auto &x : v) {
		std::cout << x << std::endl;
	}
	for (auto&& x : v) {
		x += 2;
	}
	for (const auto &x : v) {
		std::cout << x << std::endl;
	}
}