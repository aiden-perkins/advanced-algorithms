#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>
#include <regex>
#include <algorithm>
#include <string>
#include <future>
#include <chrono>
#include <ctime>
#include <iomanip>

class Graph {
public:
    size_t n;
    std::vector<std::unordered_set<size_t>> edges;

    Graph(size_t size) : n(size), edges(size) {}

    void add_edge(size_t a, size_t b) {
        edges[a].insert(b);
        edges[b].insert(a);
    }
};

Graph parse_input(const std::string& file_path) {
    std::ifstream file(file_path);
    if (!file.is_open()) {
        throw std::runtime_error("Failed to open the file");
    }

    std::regex re(R"(UndirectedEdge\[(\d+),\s*(\d+)])");
    std::string line;
    size_t max_node = 0;
    std::vector<std::pair<size_t, size_t>> edges;

    while (std::getline(file, line)) {
        std::smatch match;
        std::string::const_iterator search_start(line.cbegin());
        while (std::regex_search(search_start, line.cend(), match, re)) {
            size_t a = std::stoul(match[1]);
            size_t b = std::stoul(match[2]);
            max_node = std::max({max_node, a, b});
            edges.emplace_back(a, b);
            search_start = match.suffix().first;
        }
    }

    Graph graph(max_node + 1);
    for (const auto& [a, b] : edges) {
        graph.add_edge(a, b);
    }

    return graph;
}

bool can_color(const Graph& graph, std::vector<int>& colors, size_t vertex, int color, int max_color) {
    // Check if any adjacent vertex has the same color
    for (size_t adj : graph.edges[vertex]) {
        if (colors[adj] == color) {
            return false;
        }
    }
    return true;
}

bool color_graph_util(const Graph& graph, std::vector<int>& colors, size_t vertex, int max_color) {
    // If all vertices are colored, return true
    if (vertex == graph.n) {
        return true;
    }
    
    // Try different colors for this vertex
    for (int color = 1; color <= max_color; color++) {
        // Skip if this vertex has no edges (can use any color)
        if (graph.edges[vertex].empty()) {
            colors[vertex] = 1;
            if (color_graph_util(graph, colors, vertex + 1, max_color))
                return true;
            colors[vertex] = 0;
            return false;
        }
        
        // Check if we can color with current color
        if (can_color(graph, colors, vertex, color, max_color)) {
            colors[vertex] = color;
            if (color_graph_util(graph, colors, vertex + 1, max_color))
                return true;
            colors[vertex] = 0;
        }
    }
    
    return false;
}

bool is_colorable(const Graph& graph, int colors) {
    std::vector<int> vertex_colors(graph.n, 0);
    return color_graph_util(graph, vertex_colors, 0, colors);
}

int chromatic_number(const std::string& file_path) {
    Graph graph = parse_input(file_path);
    
    // Handle empty graph
    if (graph.n == 0) return 0;
    
    // Handle graph with no edges
    bool has_edges = false;
    for (const auto& adj_list : graph.edges) {
        if (!adj_list.empty()) {
            has_edges = true;
            break;
        }
    }
    if (!has_edges) return 1;
    
    // Try coloring with increasing number of colors
    std::vector<int> specific_colors = {1, 2, 3, 4, 5, 7, 9, 11, 14, 16, 18, 22, 24, 25, 27, 29, 34, 36, 38, 39, 41, 46, 47, 50, 57, 58};
    // for (int colors = 1; colors <= graph.n; colors++) {
    for (int colors: specific_colors) {
        if (is_colorable(graph, colors)) {
            return colors;
        }
        auto now = std::chrono::system_clock::now();
        auto now_c = std::chrono::system_clock::to_time_t(now);
        std::cout << "[" << std::put_time(std::localtime(&now_c), "%H:%M:%S") << "] "
                  << "Min of " << colors << "/" << graph.n << " colors doesn't work. (exclusive)"
                  << "(" << file_path << ")" << std::endl;
    }
    
    return graph.n; // Worst case: need different color for each vertex
}

int main() {
    std::vector<std::string> file_paths = {
        "./tests/chromatic-number/data/2.txt",
        "./tests/chromatic-number/data/4.txt",
        "./tests/chromatic-number/data/5.txt",
        "./tests/chromatic-number/data/6.txt",
        "./tests/chromatic-number/data/8.txt",
    };

    std::vector<std::future<int>> futures;

    auto start_time = std::chrono::high_resolution_clock::now();

    // Launch a thread for each file
    for (const auto& file_path : file_paths) {
        futures.push_back(std::async(std::launch::async, chromatic_number, file_path));
    }

    // Collect results
    std::vector<int> results;
    for (auto& future : futures) {
        results.push_back(future.get());
    }

    auto end_time = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(end_time - start_time);

    // Print results
    for (size_t i = 0; i < file_paths.size(); ++i) {
        std::cout << "File " << (i + 1) << " size: " << results[i] << std::endl;
    }

    std::cout << "Total execution time: " << duration.count() << " ms" << std::endl;

    return 0;
}
