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
  int a, b, operation;
  
  cout << "What operation you want to perform {1: Addition, 2: Fibonacci}: ";
  cin >> operation;

  if (operation == 1) {
    cout << "Enter first number ";
    cin >> a;

    cout << "Enter second number ";
    cin >> b;
  
    cout << "The sum of two numbers entered is: " << add(a, b) << endl;
  } else if (operation == 2) {
    
    cout << "Enter the number ";
    cin >> a;

    cout << "Fibonacci of the first number is: " << fibonacci(a) << endl;
  } else {
    cout << "Invalid operation Id";
  }
  return 0;
}
