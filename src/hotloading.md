# Mun Hotloading

Mun is build to support hot code reloading. Hot code reloading is the process of
enabling changing the code while the code is running. Having the ability to
modify running code can be a huge time safer because developers are not required
to quit the program, they can retain the application's state and avoid context
switching.

Although hot reloading can increase developer productivity it also has a cost on
the efficiency of the code. It will be harder for the compiler to do some
optimizations (like inlining) because it must be able to swap out the
implementation at any time. 

Mun has hotloading enabled by default but for release builds it the feature is
disabled to generate more efficient code. Since the ability to change running
code is generally only useful for developers and not for the end-users of the
software this seems like a sensible default. Enabling hot loading in release
builds is also supported.

On most major OSes it is possible to modify memory that contains executable
code. However, platforms with higher security concerns often do not allow
executing generated code at runtime. Examples of these platforms are iOS and
most game console platforms.

## Function hotloading

Hotloading in Mun is enabled by adding an indirection in between function calls.
Instead of jumping directly to the location of the function the address of the
function is first looked up in a registry and then the code jumps to that
location. Hotloading is thus enabled by modifying the address to jump to. 

To express what this looks like in C, consider this trivial example:

```c
#include <stdio.h>

int plus(int a, int b) {
    return a-b;
}

void printValue() {
    printf("3+4=%d\n", plus(3, 4));
}

int main()
{
    printValue();
    return 0;
}
```

Mun adds a lookup to enable modifying the implementation of `plus` at
runtime, e.g.:

```c
#include <stdio.h>

struct {
    int (*plus)(int, int);
} registry;

int plus(int a, int b) {
    return a-b;
}

int plus_fix(int a, int b) {
    return a+b;
}

void printValue() {
    printf("3+4=%d\n", (*registry.plus)(3, 4));
}

int main()
{
    registry.plus = &plus;
    printValue();
    
    registry.plus = &plus_fix;
    printValue();

    return 0;
}
```

In the example above the exact same invocation can be used with both the old and
the fixed `plus` implementation at the cost of an extra indirection. Of course
this example is not very relevant but it shows the basic principle of Mun
hotloading: patching execution pointers.

Things become more complicated when adding or removing functions because that
changes the layout of the registry. Removing functions will cause a compile
error unless all invocations are also removed. The new code will therefore run
perfectly with a registry without the old function, but any old code that might
still be running will not run with the newly compiled registry. We work around
this by adding a new registry to every compiled module and patching the old
registries. 

### Tombstone implementations

Thombstoning is the process of swapping out a function pointer for something
that when invoked will result in an error. 

# Questions

How to handle multiple threads?