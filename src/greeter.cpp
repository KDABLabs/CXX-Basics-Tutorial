#include <cxx-basics/include/greeter.h>
#include <cxx-basics/src/main.rs.h>

#include <iostream>

void Greeter::greet(const Greeting &greeting) const {
  std::cout << std::string(greeting.translate(Language::English)) << std::endl;
  std::cout << std::string(greeting.translate(Language::French)) << std::endl;
  std::cout << std::string(greeting.translate(Language::German)) << std::endl;
}

std::unique_ptr<Greeter> makeGreeter() {
  return std::make_unique<Greeter>();
}
