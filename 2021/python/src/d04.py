
class BingoBoard:

    def __init__(self, n):
        self.numbers = [int(x) for x in n.replace('\n\n',' ').split()]
        self.marked = set()
        self.cinquina = []

    def check_win(self):
        if self.check_row() or self.check_column():
            return True

    def check_row(self):
        for n in range(0,21,5):
            if self.marked.issuperset(self.numbers[n:n+5]):
                return True

    def check_column(self):
        for n in range(5):
            if self.marked.issuperset(self.numbers[n::5]):
                return True


numbers, *boards = open('../input/04.txt', 'r').read().split('\n\n')
numbers = [int(x) for x in numbers.split(',')]
boards = [BingoBoard(board) for board in boards]

last = set()
first = False

for n in numbers:
    for board in boards:
        if n in board.numbers:
            board.marked.add(n)
            if board.check_win():
                last.add(board)
                unmarked = sum(set(board.numbers) - board.marked)

                if first == False:
                    print("Part 1:", unmarked * n)
                    first = True

                if len(boards) == len(last):
                    print("Part 2:", unmarked * n)
                    exit()