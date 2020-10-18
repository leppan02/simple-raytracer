import numpy as np
import matplotlib.pyplot as plt


def solve(p=[0, 0, 0]):
    return np.roots(p)


def pythagoras(a=None, b=None, c=None):
    if a == None:
        return (c**2-b**2)**0.5
    if b == None:
        return (c**2-a**2)**0.5
    if c == None:
        return (a**2+b**2)**0.5


def intersect_relative_origin(direction, postion, radius):
    ray = direction/np.linalg.norm(direction)  # normalisera till längd 1
    dist_to_circle = np.linalg.norm(postion)
    dist = ray.dot(postion)  # avståndet till punkten mellan skärningar
    fake_r = pythagoras(c=dist_to_circle, a=dist)
    if fake_r > radius:
        return None, None
    offset = pythagoras(c=radius, a=fake_r)
    distA = dist-offset
    distB = dist+offset
    if distA <= 0:
        intersectA = None
    else:
        intersectA = distA*ray
    if distB <= 0:
        intersectB = None
    else:
        intersectB = distB*ray
    return intersectA, intersectB


def main():
    res = 200
    origin = np.array([0, 0, 0])
    direction = np.array([1.73*res*2, -1*res, -1*res])
    window = [[250]*res*2 for _ in range(res*2)]
    window[0][0] = 0
    circle = (np.array([100, 50, 0]), 20)
    for x in range(res*2):
        for y in range(res*2):
            cur_dir = direction.copy()
            cur_dir[1] += x
            cur_dir[2] += y
            point = intersect_relative_origin(cur_dir, *circle)
            window[x][y] = min(point, window[x][y])
    circle = (np.array([100, -50, 0]), 20)
    for x in range(res*2):
        for y in range(res*2):
            cur_dir = direction.copy()
            cur_dir[1] += x
            cur_dir[2] += y
            point = intersect_relative_origin(cur_dir, *circle)
            window[x][y] = min(point, window[x][y])
    window = np.array(window)
    plt.imshow(window, cmap="gray")
    plt.show()


main()

#window = np.matrix([[0 for _ in range(100)] for _ in range(100)])
#window_pos = np.matrix([50, -50, -50])
