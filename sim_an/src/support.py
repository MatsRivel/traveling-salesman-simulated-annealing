NCALLS = 130
NVEHICLES = 40

def call_info_guard_clause(line):
    if int(line[0]) >NCALLS:
        return False
    return True

def car_info_guard_clause(line):
    if int(line[0]) >NVEHICLES:
        return False
    return True

def call_and_car_info_guard_clause(line):
    car = line.strip("\n").split(",")[0]
    call = line.strip("\n").split(",")[2]
    if int(car) > NVEHICLES:
        return False
    if int(call) > NCALLS:
        return False
    return True

def dummy_func(line):
    return True 

def non_redundant_travel_costs_guard_clause(line):
    (vehicle, origin, destination, travel_time, travel_cost) = line.strip("\n").split(",")
    origin = int(origin)
    destination = int(destination)
    if origin <= destination and car_info_guard_clause(line):
        return True
    return False

def make_custom_call_vehicle_combinations():
    path = "src\\"
    input_file = f"Call_7_Vehicle_3.txt"
    output_file = f"Call_{NCALLS}_Vehicle_{NVEHICLES}_Custom.txt"

    divisor_list = []
    text_list = []
    func_list = [dummy_func,
                dummy_func,
                car_info_guard_clause,
                dummy_func,
                car_info_guard_clause,
                call_info_guard_clause,
                car_info_guard_clause,
                call_and_car_info_guard_clause
                ]
    counter = -1
    with open(path+input_file,"r") as rfile:
        inner_text_list = []
        for line in rfile.readlines():
            if line.startswith("%"):
                counter += 1
                divisor_list.append(line)
                if counter > 0:
                    text_list.append(inner_text_list)
                    inner_text_list = []
                if line.startswith("% EOF"):
                    break
            elif func_list[counter](line):
                inner_text_list.append(line)
    text_list[1] = f"{NVEHICLES}\n"
    text_list[3] = f"{NCALLS}\n"
    with open(path+output_file,"w") as wfile:
        for i in range(0,counter):
            print(text_list[i][0])
            wfile.writelines(divisor_list[i])
            wfile.writelines(text_list[i])

def remove_redundant_information():
    """
    Makes a new file, only changing the "travel times and costs" section.
    Originally its contents would assume that "a" to "b" and "b" to "a" are not equivalent.
    In our case, they are, so this function recreates the file with much less information.
    Will currently keep "a to a" type information.
    """
    path = ""
    input_file = f"Call_{NCALLS}_Vehicle_{NVEHICLES}.txt"
    output_file = f"Call_{NCALLS}_Vehicle_{NVEHICLES}_non_redundant.txt"
    print(f"{input_file}\n{output_file}\n")
    divisor_list = []
    text_list = []
    func_list = [dummy_func,
                dummy_func,
                car_info_guard_clause,
                dummy_func,
                car_info_guard_clause,
                call_info_guard_clause,
                non_redundant_travel_costs_guard_clause,
                call_and_car_info_guard_clause
                ]
    counter = -1
    with open(path+input_file,"r") as rfile:
        inner_text_list = []
        for line in rfile.readlines():
            if line.startswith("%"):
                counter += 1
                divisor_list.append(line)
                if counter > 0:
                    text_list.append(inner_text_list)
                    inner_text_list = []
                if line.startswith("% EOF"):
                    break
            elif func_list[counter](line):
                inner_text_list.append(line)
    text_list[1] = f"{NVEHICLES}\n"
    text_list[3] = f"{NCALLS}\n"
    with open(path+output_file,"w") as wfile:
        for i in range(0,counter):
            print(text_list[i][0])
            wfile.writelines(divisor_list[i])
            wfile.writelines(text_list[i])


def main():
    #make_custom_call_vehicle_combinations()
    remove_redundant_information()
    ...


if __name__ == '__main__':
    main()