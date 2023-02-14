#pragma once

#include <value.h>

class Expression {
public:
    virtual Value Evaluate() = 0;
};

