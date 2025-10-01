# Car Vision

From the LiDAR data of a car, we will display the point cloud, and try to make sense of all the data from different points of view, switching between the car and a top-down view.

## Kitti Dataset

https://www.cvlibs.net/datasets/kitti/

```
@article{Geiger2013IJRR,
  author = {Andreas Geiger and Philip Lenz and Christoph Stiller and Raquel Urtasun},
  title = {Vision meets Robotics: The KITTI Dataset},
  journal = {International Journal of Robotics Research (IJRR)},
  year = {2013}
}
```

### Prepare

```sh
uv init
uv add numpy
uv add 'laspy[lazrs]'
uv run python
```

```python
import numpy as np
import laspy
import os

for date in [date for date in os.listdir('assets/') if date.startswith('2011')]:
    for ds in [ds for ds in os.listdir('assets/' + date) if 'Store' not in ds]:
        print(ds)
        os.mkdir("assets/kitti-" + ds)
        for f in os.listdir('assets/' + date + '/' + ds + '/velodyne_points/data'):
            pc = np.fromfile('assets/' + date + '/' + ds + '/velodyne_points/data/' + f, dtype=np.float32).reshape((-1, 4))[:,:4]
            output = laspy.create()
            output.x = pc[:, 0]
            output.y = pc[:, 1]
            output.z = pc[:, 2]
            # output.intensity = pc[:, 3] * 65535
            output.red = [0.2 * 65535 for _ in range(len(pc))]
            # output.green = [0.2 * 65535 for _ in range(len(pc))]
            # output.blue = [1.0 * 65535 for _ in range(len(pc))]
            output.green = pc[:, 3] * 65535
            output.blue = (1 - pc[:, 3]) * 65535
            output.write("assets/kitti-" + ds + "/" + f.replace(".bin", ".laz"))
```

### Already Prepared

https://drive.google.com/file/d/1BDvaCX2748OcW7aPPngLRmZqTk4ms_ee/view?usp=sharing
