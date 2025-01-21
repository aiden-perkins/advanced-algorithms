import os

from typing import List, Set

def is_safe(v, graph, color, c):
    for neighbor in graph[v]:
        if color[neighbor] == c:
            return False  # If any adjacent vertex has the same color, it's not safe
    return True

def graph_coloring_util(v, graph, color, m):
    if v == len(graph):
        return True  # All vertices are colored, a solution is found

    for c in range(1, m + 1):
        if is_safe(v, graph, color, c):
            color[v] = c

            # Recur for the next vertices
            if graph_coloring_util(v + 1, graph, color, m):
                return True

            # Backtrack
            color[v] = 0

    return False  # No solution found for this coloring

def graph_coloring(graph, m):
    n = len(graph)
    color = [0] * n

    if not graph_coloring_util(0, graph, color, m):
        print("No feasible solution exists")
        return 0

    # Count unique colors to determine chromatic number
    unique_colors = set(color)
    return len(unique_colors)

if __name__ == "__main__":

    for file in os.listdir('data'):
        graph = []
        for i in range(100):
            graph.append([])
        content = open('data/' + file).read().replace('\n', '')
        edges = content.split('UndirectedEdge')[1:]
        for raw_edge in edges:
            str_nums = raw_edge.strip()[1:-2].split(', ')
            # g.add_edge(int(str_nums[0]) - 1, int(str_nums[1]) - 1)
            graph[int(str_nums[0]) - 1].append(int(str_nums[1]) - 1)
        # print(file, g.chromatic_number())
        print(graph_coloring(graph, 3))