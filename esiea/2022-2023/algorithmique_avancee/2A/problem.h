#ifndef PROBLEM_PROBLEM_H
#define PROBLEM_PROBLEM_H

#include <stddef.h>
#include <stdint.h>

typedef size_t Node;
typedef float Cost;

typedef struct
{
    Cost** matrix;
    size_t node_count;
} Graph;

typedef struct
{
    char* name;
    Cost  cost;
} Terrain;

typedef struct
{
    Terrain* terrains;
    size_t   terrain_count;
} Terrains;

void show_matrix(Graph* __graph);

Graph* create_graph(
    const char* __node_type,
    const char* __type_cost,
    const char* __terrain
);

#endif // PROBLEM_PROBLEM_H
