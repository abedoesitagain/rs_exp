
const WINNING_LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];
def show_t(t):
    for i in range(3) :
        for j in range((i * 3), (i * 3 + 3)):
            if t[j] == 2:
                print(f". ", end="");
            elif t[j] == 1:
                print(f"O ", end="");
            else:
                print(f"X ", end="");
        print("");

def prompt(t, turn):
    condition = False
    while not condition:
        if turn == 0:
            buff = int(input(f"X: where do you want to put this? [1..9]: "))
        else:
            buff = int(input(f"O: where do you want to put this? [1..9]: "))
        condition = t[buff-1] == 2
    t[buff-1] = turn
    return buff-1

def ptable_add(p1, p2, p, choice, i):
    global c1, c2
    if p == 0:
        p1.append(choice)
        c1 += 1
    else:
        p2.append(choice)
        c2 += 1

def tb(n, T):
    condition=True
    while condition:
        condition=False
        for i in range(n-1):
            if T[i] > T[i+1]:
                aux = T[i]
                T[i] = T[i+1]
                T[i+1] = aux
                condition=True

WINNING_LINES = [
    (0, 1, 2), (3, 4, 5), (6, 7, 8),  # rows
    (0, 3, 6), (1, 4, 7), (2, 5, 8),  # columns
    (0, 4, 8), (2, 4, 6),             # diagonals
]
def check_won(t):
    positions = set(t)
    return any(all(pos in positions for pos in line) for line in WINNING_LINES)

table = [2] * 9
p1 = [2] * 0
p2 = [2] * 0
c1 = c2 = 0

won = False

i=0
while not won:
    p = i%2
    choice=prompt(table, p)
    ptable_add(p1, p2, p, choice, i)
    if i>3:
        tb(c1, p1)
        tb(c2, p2)
        if check_won(p1):
            won=True
            winner="X"
        else:
            won=check_won(p2)
            winner="O"
    show_t(table);
    i+=1

print(f"p1:\n {p1}\n of {c1} elements")
print(f"p2:\n {p2}\n of {c2} elements")
print(f"player {winner} won!")
