#pragma once

#include <stdint.h>

enum Type {
    INT,
    UINT,
    FLOAT,
    STRING,
    FUNC,
    OBJECT
};

struct Value {
    Type type;
    intptr_t val;
    bool is_owner;
};