"""
Create dict of dicts
cd will update a path var, used as context
ls doesn't really tell us much

if not a command use path to find place in dict and add values
"""
def read_input():
    with open('real.txt', 'r') as f:
        return(f.readlines())

def handle_command(path, command): # mainly used for cd
    args =  command.split(" ")
    if args[1] == "cd": # update path logic
        if args[2] == "/":
            path = "/"
        elif args[2] == "..":
            path = "/".join(path.split("/")[:-1])
        else:
            path = path + f"/{args[2]}"
    
    return path

def add_value(path, file_system, entry):
    size, name = entry.split(" ")
    split_path = path.split("/")
    scope = file_system

    for directory in split_path:
        if( directory == ""): # goofy ahhh
            continue
        scope = scope[directory]

    # scope is now the dictionary of folder we're in
    if size == "dir":
        scope[name] = {}
    else:
        scope[name] = size
    
    return file_system

def sum_folders(folder, total_sum, folder_sizes):
    folder_sum = 0
    for item in folder:

        if type(folder[item]) == type({}):
            subfolder_sum, total_sum, folder_sizes = sum_folders(folder[item], total_sum, folder_sizes)
            folder_sum += subfolder_sum
        else:
            folder_sum += int(folder[item])

    if folder_sum < 100000:
        total_sum += folder_sum

    folder_sizes.append(folder_sum)
    return folder_sum, total_sum, folder_sizes
    

def main():
    inputs = read_input()

    path = "/"
    file_system = {}

    for line in inputs:
        line = line.strip()
        if line[0] == "$": # command is executed
            path = handle_command(path, line)
        else: # must be output from a command
            file_system = add_value(path, file_system, line)
    
    used, part_1, every_folder = sum_folders(file_system, 0, [])
    #print(used) 
    # used space is 49192532, total size 70000000
    # 70000000 - 49192532 = 20807468 free space
    # need 30000000 - 20807468 = 9192532 minimum
    print(part_1)
    #print(every_folder)

    for size in sorted(every_folder):
        if size >= 9192532:
            print(size)
            break

if __name__ == "__main__":
    main()