def main():
    print(max((len_cycle(i), i) for i in range(2,999)))
    print(len_cycle(333))

def len_cycle(div):
    num = str(1/div)[2:]

    for i in range(len(num)):
        for j in range(i+1,int(len(num[i:])/2)):
            if num[i:j+1] == num[j+1:(j+1)*2]and not all_chars_are_equal(num[i:j+1]):
                print(num[i:j+1])

"""    return max((len(num[i:j+1])for i in range(len(num))
        for j in range(i+1,int(len(num[i:])/2))
        if num[i:j+1] == num[j+1:(j+1)*2]and not all_chars_are_equal(num[i:j+1])),default=0) 

"""   
    

                
def all_chars_are_equal(number):
    return all(number[0]==number[i] for i in range(len(number)))


if __name__ == '__main__':
    main()

