
#include <iostream>
#include <fstream>
#include <vector>
#include <unordered_set>
#include <regex>
#include <algorithm>
#include <string>
#include <future>
#include <chrono>

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

void backtrack(const Graph& graph, std::unordered_set<size_t>& current_set,
               size_t start, std::unordered_set<size_t>& max_set,
               const std::string& file_path) {
    if (current_set.size() > max_set.size()) {
        max_set = current_set;
        std::cout << file_path << ": " << max_set.size() << std::endl;
    }

    for (size_t v = start; v < graph.n; ++v) {
        bool can_add = true;
        for (size_t u : current_set) {
            if (graph.edges[u].count(v) > 0) {
                can_add = false;
                break;
            }
        }
        if (can_add) {
            current_set.insert(v);
            backtrack(graph, current_set, v + 1, max_set, file_path);
            current_set.erase(v);
        }
    }
}

std::unordered_set<size_t> find_maximum_independent_set(const Graph& graph, const std::string& file_path) {
    std::unordered_set<size_t> max_set, current_set;
    backtrack(graph, current_set, 0, max_set, file_path);
    return max_set;
}

int independent_set(const std::string& file_path) {
    Graph graph = parse_input(file_path);
    std::unordered_set<size_t> max_set = find_maximum_independent_set(graph, file_path);
    return static_cast<int>(max_set.size());
}

int main() {
    std::vector<std::string> file_paths = {
        "./tests/independent-set/data/1.txt",
        "./tests/independent-set/data/2.txt",
        "./tests/independent-set/data/3.txt",
        "./tests/independent-set/data/4.txt",
        "./tests/independent-set/data/5.txt",
        "./tests/independent-set/data/6.txt",
        "./tests/independent-set/data/7.txt",
        "./tests/independent-set/data/8.txt",
        "./tests/independent-set/data/9.txt",
        "./tests/independent-set/data/10.txt"
    };

    std::vector<std::future<int>> futures;

    auto start_time = std::chrono::high_resolution_clock::now();

    // Launch a thread for each file
    for (const auto& file_path : file_paths) {
        futures.push_back(std::async(std::launch::async, independent_set, file_path));
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
        std::cout << "File " << (i + 1) << " maximum independent set size: " << results[i] << std::endl;
    }

    std::cout << "Total execution time: " << duration.count() << " ms" << std::endl;

    return 0;
}