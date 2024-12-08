"""
https://new.reddit.com/r/adventofcode/comments/1h8nqxa/comment/m0xdgl5/

 Suppose you have four kids, let's call them A, B, C, and D, and a motorcycle.
 You can only transport one kid at a time. The number of possible passenger 
 combinations is 4: you can carry {A}, {B}, {C}, or {D}.

You and your spouse upgrade the motorcycle with a sidecar. Now you can carry two
kids. The number of passenger combinations is now 6: {A,B}, {A,C}, {A,D}, {B,C},
 {B,D}, and {C,D}.

Not satisfied, you buy a little hatchback that can carry three kids. Perhaps
unintuitively, the number of passenger combinations drops to 4 (but intuitively,
 you just have to choose which kid doesn't ride along): {A,B,C}, {A,B,D},
 {A,C,D}, {B,C,D}.

You trade the hatchback for a van and can transport all four kids at once. There
is only one possible passenger combination now: {A,B,C,D}.

You crash the van into the motorbike and are left with only a bicycle. Now you
cannot carry any kids. There's only one way you can carry zero passengers: {}.
"""

def combine(passengers, capacity, decisions):
    if capacity == 0:
        return [passengers]
    if not decisions:
        return []
    candidate = decisions[0]
    decisions = decisions[1:]
    p1 = passengers
    p2 = passengers.copy()
    p2.append(candidate)
    return [*combine(p1, capacity, decisions), *combine(p2, capacity-1, decisions)]

for r in range(0,5):
    print(r, combine([], r, ["A", "B", "C", "D"]))
