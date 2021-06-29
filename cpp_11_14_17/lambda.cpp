// #include <iostream>
// #include <functional>

// void ReadImage(std::string &filePath, const std::function::<void(double)> &func) {
// 	std::cout << filePath << std::endl;
// 	for (int y = 0; y < 30; ++y) {
// 		for (int x = 0; x < 30; ++x) {
// 			func(3.0);
// 		}
// 	}
// }

// int main() {
// 	ReadImage("test.jpg", [&](double v) {
// 		std::cout << v << std::endl;
// 	});
// }