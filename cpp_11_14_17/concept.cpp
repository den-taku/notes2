template<typename T>
concept bool Addable = requires (T x) { x + x; };

template<typename T> requires Addable<T>
T add(T a, T b) { return a + b; }

int main() {}