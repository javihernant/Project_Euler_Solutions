cycles=[]

def main():
    #print (max(len_cycle(i)for i in range(2,1000)))
    i_new=0
    max_v_tuple =(0,"")

#    for i in range(2,1000):
#        if max(len_cycle(i), default=0)> max_v_tuple:
#            max_v_tuple=max(len_cycle(i), default=0)
#            i_new = i
#
    for i in range(2,1000):
        if max(len_cycle(i),default=(0,""))>max_v_tuple:
            max_v_tuple = max(len_cycle(i),default=0)
            i_new = i

    print(max_v_tuple, i_new)




def len_cycle(div):
    num = str(1/div)[2:]
    
    global cycles
    for i in range(len(num)):
        cycles.clear()
        for j in range(i+1,int(len(num[i:])/2)):
            if num[i:j+1] == num[j+1:(j+1)*2]and not has_smaller_cycles(num[i:j+1]):
                yield (len(num[i:j+1]),num[i:j+1])

    
#    return max((len(num[i:j+1])for i in range(len(num))
#        for j in range(i+1,int(len(num[i:])/2)) 
#        if num[i:j+1] == num[j+1:(j+1)*2]and not has_smaller_cycles(num[i:j+1])),default=0) 
#
   
    

                
def has_smaller_cycles(new_cycle):
    global cycles
    if not any(c for c in cycles if c in new_cycle):
        cycles.append(new_cycle) 
        return False
    else:
        return True


if __name__ == '__main__':
    main()


