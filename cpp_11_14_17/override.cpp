#include <iostream>

class A {
    public:
        A() { std::cout << "A" << std::endl; }
    public:
        void hoge() { std::cout << "A::hoge" << std::endl; }
};

class B : A {
    public:
        B() { std::cout << "B" << std::endl; }
    public:
        void hoge() { std::cout << "B::hoge" << std::endl; }
};

int main() {
    B b;
    b.hoge();
}
