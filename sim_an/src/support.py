NCALLS = 4
NVEHICLES = 2

def call_info_guard_clause(line):
    if int(line[0]) >NCALLS:
        return False
    return True

def car_info_guard_clause(line):
    if int(line[0]) >NVEHICLES:
        return False
    return True

def call_and_car_info_guard_clause(line):
    car = line[0]
    call = line[2]
    if int(car) > NVEHICLES:
        return False
    if int(call) > NCALLS:
        return False
    return True

def dummy_func(line):
    return True 

def main():
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

if __name__ == '__main__':
    main()