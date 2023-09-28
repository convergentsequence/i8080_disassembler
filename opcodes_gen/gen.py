with open("opcodes", "r") as f:
    data = f.read().split("\n")
    data = list(map(lambda l: l.split("\t")[:3], data))
    data = [ list(map(lambda x: x.strip(), l)) for l in data ]

    for l in data:
        try:
            print(f"opcode!(\"{l[1]}\", {l[0]}, {l[2]}),")
        except:
            pass
