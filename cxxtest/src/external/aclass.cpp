#include "headers/aclass.hpp"
#include <iostream>

std::unique_ptr<AClass> new_aclass(uint32_t v) {
  return std::unique_ptr<AClass>(new AClass(v));
}

AClass::AClass(uint32_t p_value) : a_value(p_value) {}

void AClass::process() const {
  std::cout << "Processing " << this->a_value << std::endl;
}
