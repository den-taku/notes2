#include <iostream>

class A {
    public:
        A() { std::cout << "A" << std::endl; }
    public:
        virtual void vrt() { std::cout << "A::vtr" << std::endl; }
        void nvtr() { std::cout << "A::nvtr" << std::endl; }
};

class B : A {
    public:
        B() { std::cout << "B" << std::endl; }
    public:
        void vrt() { 
            A::vrt();
            std::cout << "B::vtr" << std::endl; 
        }
        void nvtr() { 
            A::nvtr();
            std::cout << "B::nvtr" << std::endl; 
        }
        // void nvtr();
};

int main() {
    B b;
    b.vrt();
    b.nvtr();
}
