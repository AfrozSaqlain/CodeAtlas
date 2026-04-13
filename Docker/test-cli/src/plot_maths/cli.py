import numpy as np
import matplotlib.pyplot as plt

def main():
    x = np.arange(10)
    y = x**2


    plt.plot(x, y)
    plt.xlabel('X')
    plt.ylabel('Y')
    plt.savefig("Plot.jpg")

if __name__ == '__main__':
    main()
