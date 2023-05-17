#include "problem.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void make_link(Graph* __graph, Node __source, Node __target, Cost __cost)
{
    __graph->matrix[__source][__target] == __cost;
}

void show_matrix(Graph* __graph)
{
    Cost** graph_end = (__graph->matrix + __graph->node_count);

    for (Cost** i = __graph->matrix; i != graph_end; ++i) {
        Cost* line_end = (*i + __graph->node_count);

        for (Cost* j = *i; j != line_end; ++j) {
            printf("%f ", *j);
        }

        printf("\n");
    }
}

void print_failed_opening_file(const char* filename)
{
    fprintf(stderr, "The file '%s' cannot be openned.\n", filename);
}

char** read_node_type(const char* filename, size_t node_count)
{
    char** ret = NULL;

    FILE* file = fopen(filename, "r");

    if (file == NULL) {
        print_failed_opening_file(filename);
    } else {
        ret = malloc(sizeof(char*) * node_count);

        for (; node_count != 0; --node_count) {
            char type[16];
            Node node;

            fscanf(file, "%lu %s", &node, type);

            ret[node] = malloc(sizeof(char) * (strlen(type) + 1));
            strcpy(ret[node], type);
        }

        fclose(file);
    }

    return ret;
}

Terrains* read_type_cost(const char* filename)
{
    Terrains* ret = NULL;

    FILE* file = fopen(filename, "r");

    if (file == NULL) {
        print_failed_opening_file(filename);
    } else {
        ret = malloc(sizeof(Terrain));

        fscanf(file, "%lu", &ret->terrain_count);

        ret->terrains = malloc(sizeof(Terrain) * ret->terrain_count);

        Terrain* end = (ret->terrains + ret->terrain_count);

        for (Terrain* i = ret->terrains; i != end; ++i) {
            i = malloc(sizeof(Terrain));

            char name[16];

            fscanf(file, "%s %f", name, &i->cost);

            i->name = malloc(sizeof(char) * (strlen(name) + 1));
            strcpy(i->name, name);
        }

        fclose(file);
    }

    return ret;
}

float find_cost_from_terrain_name(Terrains* __terrains, const char* __name)
{
    Terrain* end = (__terrains->terrains + __terrains->terrain_count);

    for (Terrain* i = __terrains->terrains; i != end; ++i) {
        if (strcmp(i->name, __name) == 0) {
            return i->cost;
        }
    }

    fprintf(stderr, "The terrain called '%s' does not exist\n", __name);

    return -1.;
}

Graph* init_graph(size_t __node_count)
{
    Graph* ret = malloc(sizeof(Graph));

    ret->node_count = __node_count;
    ret->matrix     = malloc(sizeof(Cost*) * __node_count);

    Cost** end = ret->matrix + __node_count;

    for (Cost** i = ret->matrix; i != end; ++i) {
        *i = malloc(sizeof(Cost*) * __node_count);
    }

    return ret;
}

Graph* create_graph(
    const char* __node_type,
    const char* __type_cost,
    const char* __terrain
)
{
    Graph* ret = NULL;

    FILE* file = fopen(__terrain, "r");

    if (file == NULL) {
        print_failed_opening_file(__terrain);
    } else {
        Node begin, end;

        fscanf(file, "DÉBUT: %lu", &begin);
        fscanf(file, "FIN: %lu", &end);

        ret = init_graph(end - begin + 1);

        char** node_type    = read_node_type(__node_type, ret->node_count);
        Terrains* type_cost = read_type_cost(__type_cost);

        Node node_a, node_b;
        char arrow[2];

        while (fscanf(file, "%lu %s %lu", &node_a, arrow, &node_b) > 0) {
            Node src, dest;

            if (arrow[0] == '-') {
                src  = node_a;
                dest = node_b;
            } else {
                src  = node_b;
                dest = node_a;
            }

            make_link(
                ret,
                src,
                dest,
                find_cost_from_terrain_name(type_cost, node_type[src])
            );
        }

        fclose(file);
    }

    return ret;
}

int main()
{
    Graph* graph = create_graph(
        "typeSommet1.txt",
        "type1.txt",
        "terrain1.txt"
    );

    if (graph == NULL) {
        fprintf(stderr, "The graph creation has failed.\n");
    } else {
        show_matrix(graph);
    }
}
