from decimal import *

def main():
    i_new=0
    max_v_tuple =(0,"")

    for i in range(2,1000):
        if max(len_cycle(i),default=(0,"")) > max_v_tuple:
            max_v_tuple = max(len_cycle(i), default=0)
            i_new = i

    print('cycle: {}\nlen:{}\ndivisor: {} '.format(max_v_tuple[1], max_v_tuple[0], i_new))




def len_cycle(div):
    getcontext().prec= 2000
    largest_cycle_saved = 0 
    num = str(1/Decimal(div))[2:]
    
    
    global cycles
    for i in range(len(num)):
        skip_div=None
        for j in range(i+1,int(len(num[i:]) / 2)):
            if j-i+1 < largest_cycle_saved:
                continue
            if num[i:j+1] == num[j+1:(j+1)*2]:

                if num[i:int((j+1)/2)] == num[int((j+1)/2):j+1] and skip_div == None:
                    skip_div = len(num[i:int((j+1) / 2)])

                if skip_div != None and len(num[i:j+1]) % skip_div == 0:
                    continue

                if largest_cycle_saved < len(num[i:j+1]):
                    largest_cycle_saved = len(num[i:j+1])

                yield (len(num[i:j + 1]),num[i:j + 1])


if __name__ == '__main__':
    main()


