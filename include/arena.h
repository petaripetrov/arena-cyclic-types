#ifndef AC_ARENA_ALLOCATOR_H
#define AC_ARENA_ALLOCATOR_H

#include <cassert>
#include <cstdlib>
#include "rust/cxx.h"
using namespace std;

template <class T>
class Arena
{
protected:
    T *entries;
    size_t current;
    size_t space_left;

public:
    Arena(const size_t size)
    {
        space_left = size;
        entries = static_cast<T *>(calloc(size, sizeof(T)));
        current = 0;
    }

    ~Arena()
    {   
        free(entries);
    }

    inline T *allocate()
    {   
        assert(space_left > 0);
        T *ptr = entries + current;
        current += 1;
        space_left -= 1;
        return ptr;
    }
};

void benchmark_linked_list_arena(size_t n);
void benchmark_linked_list_manual(size_t n);
void benchmark_tree_arena(size_t n, rust::Vec<uint32_t> arr);
void benchmark_tree_manual(rust::Vec<uint32_t> arr);

#endif