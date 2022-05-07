import datetime

def getInternetTimeString():
    dtnow = datetime.datetime.utcnow()
    beats = 0.0
    beats += ((dtnow.hour +1.0)%24) * 3600
    beats += dtnow.minute * 60
    beats += dtnow.second
    beats /= 86.4
    return "@"+str(round(beats,1))

def main():
    print(getInternetTimeString())

if __name__ == '__main__':
    main()