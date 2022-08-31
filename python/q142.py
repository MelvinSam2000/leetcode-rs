from typing import *

class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

"""
    142 - Linked List Cycle II
    Time: O(n)
    Space: O(1)
    Note: Uses Floyds Algorithm
"""
def detectCycle(self, head: Optional[ListNode]) -> Optional[ListNode]:
    cycleFound = False
    slow, fast = head, head
    while fast and fast.next:
        slow = slow.next
        fast = fast.next.next
        if slow == fast:
            cycleFound = True
            break
    if not cycleFound:
        return None
    slow2 = head
    while slow != slow2:
        slow = slow.next
        slow2 = slow2.next
    
    return slow