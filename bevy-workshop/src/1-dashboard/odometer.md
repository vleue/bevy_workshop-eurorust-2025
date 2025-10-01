# Odometer

Solutions for those exercises are behind the `odometer` feature.

## Displaying the Distance Traveled

Now that our car is moving, it would be nice to display the distance traveled. We can do this by displaying text and updating it every frame.

Tips:

- Use the [`Text2d` component](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Text2d.html) to display the distance traveled
- An `Odometer` marker component can help selecting the correct entity when updating
- Add a `Distance` resource to track the distance traveled
- Add a system that will update the distance according to the speed and the time elapsed since the last update
  - The [`Time` resource](https://docs.rs/bevy/0.17.2/bevy/prelude/struct.Time.html) can give you the time delta since the last update
- Add a system that will update the text displayed with the distance traveled
