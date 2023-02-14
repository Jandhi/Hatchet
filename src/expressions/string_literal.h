#pragma once

#include <string>
#include "expression.h"

class StringLiteral: public Expression {
    std::string contents;

public:    
    StringLiteral(std::string contents) : contents(contents) {

    }

    Value Evaluate() override;
};