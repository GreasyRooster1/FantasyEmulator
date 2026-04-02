


machine_code = []


with open('example.txt', 'r') as file:
    for line in file:
        print(line.strip())
        words = line.split()


