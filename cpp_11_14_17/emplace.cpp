#include <vector>
#include <iostream>

class Int {
    private:
        int m;
    public:
        Int(int n) : m(n){ }
    public:
        int member() { return m; }
};

int main() {
    Int e1 = Int(1);
    Int e2 = Int(2);
    Int e3 = Int(3);
    Int e4 = Int(4);
    Int e5 = Int(5);
    Int e6 = Int(6);
    Int e7 = Int(7);

    std::vector<Int*> v = {&e1, &e2, &e3, &e4, &e5, &e6, &e7};

    for (auto e : v) {
        std::cout << e -> member() << " "; 
    }
    std::cout << std::endl;

    std::vector<Int*> u;
    for (auto e : v) {
        if (e -> member() % 2 == 0) 
            u.emplace_back(e);
    }
    for (auto e : u) {
        delete e;
    }

    for (auto e : v) {
        // Segment fault occuer.
        std::cout << e -> member() << " "; 
    }
    std::cout << std::endl;

}
