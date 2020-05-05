import sys
import numpy as np
import matplotlib.pyplot as plt

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Error: specify a filename.")
        exit(1)
    filename = sys.argv[1]
    wave = None
    try:
        wave = np.fromfile(filename, dtype='uint8')
    except Exception:
        print("Error reading file - does it exist?")
        exit(1)
    num_samples = wave.shape[0]
    x_axis = np.arange(0, num_samples, 1)

    plt.plot(x_axis, wave)
    plt.title("Wave: " + filename)
    plt.xlabel('sample')
    plt.ylabel('amplitude')
    plt.show()
