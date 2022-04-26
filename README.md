# acoustics-simulator
Simulation of audio waves by solving the Partial Differential Equation (PDE) of the wave-function.

## Dependencies
- optional: ffmpeg (in case the output videos shall be merged to a video)

## Building and starting
To build and start with cargo
```Shell
make
```

## Create a video
For merging output images into a single video
```Shell
make output.avi
```

## Example Output Image
This is one of the created images:
![output](https://raw.githubusercontent.com/Luz/acoustics-simulator/master/output.png)

## Example Output Video
This is the merged video:
![output](https://raw.githubusercontent.com/Luz/acoustics-simulator/master/output.mp4)
