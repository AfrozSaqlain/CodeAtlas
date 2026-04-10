#include <iostream>
using namespace std;

int add(int a, int b) {
  return a + b;
}

int fibonacci(int a) {
  if (a <= 1) {
    return a;
  }
  else{
    return fibonacci(a - 1) + fibonacci(a - 2); 
  }
}

int main() {
  int a, b;

  cout << "Enter first number ";
  cin >> a;
  cout << "Enter second number ";
  cin >> b;
  
  cout << "The sum of two numbers entered is: " << add(a, b) << endl;

  cout << "Fibonacci of the first number is: " << fibonacci(a) << endl;
  return 0;
}
