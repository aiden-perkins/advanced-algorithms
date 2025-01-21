mx = 0

def graphSets(graph):
    global mx
    
    if len(graph) == 0:
        return []
    
    if len(graph) == 1:
        return [list(graph.keys())[0]]
    
    vCurrent = list(graph.keys())[0]
    
    graph2 = dict(graph)
    
    del graph2[vCurrent]
    
    res1 = graphSets(graph2)
    
    for v in graph[vCurrent]:
        
        if(v in graph2):
            del graph2[v]
    
    res2 = [vCurrent] + graphSets(graph2)
    if mx < len(res2):
        print(len(res2))
        mx = len(res2)
    
    if(len(res1) > len(res2)):
        return res1
    return res2


E = []
c = open('../tests/independent-set/data/1.txt').read()
for l in c.split('\n'):
    nn = l[1:].split('UndirectedEdge')[1:]
    for pair in nn:
        pair = pair.replace(']', '').replace('[', '').replace('}', '').split(', ')
        E.append((int(pair[0]), int(pair[1])))

V = 101

graph = dict([])

for i in range(len(E)):
    v1, v2 = E[i]
    
    if(v1 not in graph):
        graph[v1] = []
    if(v2 not in graph):
        graph[v2] = []
    
    graph[v1].append(v2)
    graph[v2].append(v1)

maximalIndependentSet = graphSets(graph)
