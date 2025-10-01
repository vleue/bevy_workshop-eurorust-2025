# Speedometer

Solutions for those exercises are behind the `speedometer` feature.

## Displaying the Speedometer

Display the speedometer images, with the images available in the `assets/speedometer` folder.
Tips:

- The hand will need a position relative to the dial
  - It will need to rotate around the center of the dial which is not the center of the image
  - The hand itself is too large and needs to be scaled down
  - It will rotate not around the center of itself, but around the dot in the image

Hierarchy and positions that can be used:

```ignore
[
  (dial image, default position),
  (hand group, (0.0, -125.0, 0.0) with scale (0.5),
    [
      (hand image, (0.0, 150.0, 0.0))
    ]
  )
]
```

## Making the Car Move

While the space bar is pressed, the car speed should increase. The speedometer hand should point to the correct speed.

- Add a `Resource` that will hold the speed (a `f32`)
  - Don't forget to initialize the resource in the application
- Add a `System` that will increase the speed when the space bar is pressed, and decrease it when the space bar is not pressed
- Add a `System` that will rotate the hand based on the speed
  - You will need to rotate the hand group from above, not the hand image. A marker component can help you select the correct entity
  - Rotation is done with [`Quaternion`s](https://docs.rs/bevy/0.17.2/bevy/math/struct.Quat.html)
  - `Quat::from_rotation_z(-speed.0 / 160.0 * 3.0 + 1.5)` works well for the speedometer hand rotation
