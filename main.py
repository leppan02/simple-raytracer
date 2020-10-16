import numpy as np
import matplotlib.pyplot as plt 


def solve(p = [0,0,0]):
	return np.roots(p)

def intersect(origin, direction, object_postion, object_size):
	radius = object_size**2
	ray = direction/np.linalg.norm(direction)
	circle = np.array(object_postion)
	dist_to_circle = np.linalg.norm(circle)
	dist = ray.dot(circle)
	r = (dist_to_circle**2)-(dist**2)
	if r > radius:
		return 250
	offset = (radius-r)**0.5
	return int(int(dist-offset)*2.5)
	
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
			cur_dir[1]+=x
			cur_dir[2]+=y
			point = intersect(origin, cur_dir, *circle)
			window[x][y] = min(point,window[x][y])
	circle = (np.array([100, -50, 0]), 20)
	for x in range(res*2):
		for y in range(res*2):
			cur_dir = direction.copy()
			cur_dir[1]+=x
			cur_dir[2]+=y
			point = intersect(origin, cur_dir, *circle)
			window[x][y] = min(point,window[x][y])
	window = np.array(window)
	plt.imshow(window, cmap="gray")
	plt.show()



main()

#window = np.matrix([[0 for _ in range(100)] for _ in range(100)])
#window_pos = np.matrix([50, -50, -50])
	
