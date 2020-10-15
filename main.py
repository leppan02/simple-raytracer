import numpy as np

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
    return 99
  offset = (radius-r)**0.5
  return int(dist-offset)
  
def main(): 
  origin = np.array([0, 0, 0])
  direction = np.array([25, -15, -15])
  circle = (np.array([60, 10, 10]), 20)
  window = [[9]*30 for _ in range(30)]
  for x in range(30):
    for y in range(30):
      cur_dir = direction.copy()
      cur_dir[1]+=x
      cur_dir[2]+=y
      point = intersect(origin, cur_dir, *circle)
      window[x][y] = point
  circle = (np.array([60, -20, -10]), 20)
  for x in range(30):
    for y in range(30):
      cur_dir = direction.copy()
      cur_dir[1]+=x
      cur_dir[2]+=y
      point = intersect(origin, cur_dir, *circle)
      window[x][y] = min(point,window[x][y])
  circle = (np.array([100, 0, 0]), 50)
  for x in range(30):
    for y in range(30):
      cur_dir = direction.copy()
      cur_dir[1]+=x
      cur_dir[2]+=y
      point = intersect(origin, cur_dir, *circle)
      window[x][y] = min(point,window[x][y])
  for i in window:
    for j in i:
      print(j,end='')
    print('')



main()

#window = np.matrix([[0 for _ in range(100)] for _ in range(100)])
#window_pos = np.matrix([50, -50, -50])
  
