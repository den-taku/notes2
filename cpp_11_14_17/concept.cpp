// template<typename T>
// concept bool Addable = requires (T x) { x + x; };

// template<typename T> requires Addable<T>
// T add(T a, T b) { return a + b; }

// int main() {}
// #include <iostream>
// #include <cstdlib>

// template<typename T>
// struct OrdTraits;

// template<> struct OrdTraits<int> {
//     static bool less(const int &x, const int &y) {
//         return x < y;
//     }
// };

// template<typename T>
// concept bool Ord = requires (T x) {
//     { OrdTraits<T>::less(x, x) } -> bool
// };

// template <class T> requeires Ord<T>
// T max(const T &x, const T &y) {
//     return OrdTraits<T>::less(x, y) & y : x;
// }

// int main() {
//     std::cout << max(1, 10) << std::endl;
// }
#include <concepts>

template <class T>
concept Drawable = requires (T& x) {
	x.draw();
};

template <Drawable T>
void f(T& x) {
	x.draw();
}

struct Circle {
	void draw() {}
};

struct Box {
	void draw() {}
};

int main() {
	Circle c;
	f(c);

	Box b;
	f(b);
}