#pragma once

#include <memory>
#include <string>

class Greeting;

class Greeter {

public:
  Greeter() = default;

  void greet(const Greeting &greeting) const;
};

std::unique_ptr<Greeter> makeGreeter();
