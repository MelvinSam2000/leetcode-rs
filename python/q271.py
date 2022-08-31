from typing import *

class Solution:
    
    def encode(self, words: List[str]) -> str:
        return "".join([str(len(word)) + "#" + word for word in words])

    def decode(self, w: str):
        i = 0
        res = []
        while i < len(w):
            k = 0
            while w[i] != "#":
                k *= 10
                k += int(w[i])
                i += 1
            i += 1
            res.append(w[i:i + k])
            i += k
        return res

