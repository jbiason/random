#ifndef ACLASS_HPP
#define ACLASS_HPP

#include <memory>
#include <stdint.h>

class AClass {
private:
  uint32_t a_value;

public:
  AClass(uint32_t p_value);

  void process() const;
};

std::unique_ptr<AClass> new_aclass(uint32_t v);

#endif
