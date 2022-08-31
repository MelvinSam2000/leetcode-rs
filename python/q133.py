from typing import *

class Node:
    def __init__(self, val = 0, neighbors = None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []

"""
    133 - Clone Graph
    Time: O(V + E)
    Space: O(V)
"""
def cloneGraph(self, node: 'Node') -> 'Node':
    m = {}
    
    def dfs(node):
        if node in m:
            return m[node]
        
        copy = Node(node.val)
        m[node] = copy
        for nei in node.neighbors:
            copy.neighbors.append(dfs(nei))
        return copy
    
    return dfs(node) if node else None