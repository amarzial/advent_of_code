data = open("../input/16.txt", 'r').read()
line = bin(int(data,16))[2:].zfill(len(data)*4)

def parse_litteral(data):
    conv = ''
    i = 0
    while data[i] != '0':
        conv += data[i+1:i+5]
        i += 5
    conv += data[i+1:i+5]
    return int(conv, 2), i+5


def parse_pakage(data, tree, i=0):
    version = int(data[i:i+3], 2)
    typeID = int(data[i+3:i+6], 2)
    tree['ver'] += version

    if typeID != 4:
        if data[i+6] == '0':
            length = int(data[i+7:i+22], 2)
            pos = i + 22
            while pos < i + 22 + length:
                pos = parse_pakage(data, tree,pos)
            return pos
        else:
            n_sub = int(data[i+7:i+18], 2)
            pos = i + 18
            for k in range(n_sub):
                pos = parse_pakage(data, tree, pos)
            return pos

    # Litteral
    else:
        value, pos = parse_litteral(data[i+6:])
        return i+6+pos

tree = {'ver': 0}
parse_pakage(line, tree)
print(f"Part 1: {tree['ver']}")