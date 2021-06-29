#define rep(i, n) for(int i = 0; i < (int)(n); ++i)
#include <iostream>
#include <memory>

class hoge {
	private:
		std::unique_ptr<int> ptr;
	public:
		hoge(int val_): ptr(new int(val_)) {}
		int getValue() const { return *ptr; }
};

int main() {
	hoge Hoge(10);
	// hoge Hoge2(Hoge);
	hoge Hoge2(std::move(Hoge));

	std::unique_ptr<int> ptr(new int(10));

	std::unique_ptr<int> ptr2;
	ptr2.reset(new int(10));

	std::unique_ptr<int> ptr3=std::make_unique<int>(10);

	std::cout << *ptr << ", " << *ptr2 << ", " << *ptr3 << std::endl;

	std::unique_ptr<int> ptr4(std::move(ptr));
	std::unique_ptr<int> ptr5;
	ptr5 = std::move(ptr4);

	std::cout << *ptr5 << std::endl;

	ptr5.reset();

	// std::cout << *ptr5 << std::endl;


	// std::auto_ptr<int> ptr(new int(10));

	// for(int i = 0; i < 10; ++i ) {
	// 	*ptr += i;
	// }

	// std::cout << "ptr = " << *ptr << std::endl;

	// delete ptr;

	return 0;
}
