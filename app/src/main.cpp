#include <iostream>
#include "lib.hpp"
using namespace std;

int main(int argc, char* args[]) {
    cout << "Hello, World!" << endl;
    cout << "Added 2+2 via extern libary result: " << add(2, 2) << endl;
    return 0;
}