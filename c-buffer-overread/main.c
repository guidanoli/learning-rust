#include <stdlib.h>
#include <assert.h>
#include <stdio.h>

int main(int argc, char** argv)
{
    int i;
    int v[1] = {0};
    assert(argc >= 2 && "Usage: <program> <index>");
    i = atoi(argv[1]);
    printf("%d\n", v[i]);
    return 0;
}
