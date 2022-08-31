from typing import *

class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

"""
    141 - Linked List Cycle
    Time: O(n)
    Space: O(1)
    Note: Uses Floyds Algorithm
"""
def hasCycle(self, head: Optional[ListNode]) -> bool:
    slow = head
    fast = head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        if slow == fast:
            return True
    return False