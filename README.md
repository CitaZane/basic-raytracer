# RT

## Project description
Ray tracing is a technique for modeling light transport for generating digital images. For this simple [ray tracing project](https://github.com/01-edu/public/tree/master/subjects/rt) it is possible to generate 4 different objects : Sphere, Cube, Plane and Cylinder.

By changing coordinates you can also change camera location and light object position.

Digital pictures are in [ppm format](https://www.adobe.com/creativecloud/file-types/image/raster/ppm-file.html). Most convinient way of viewing them is by using VSCode [extension](vscode:extension/martingrzzler.simple-ppm-viewer)

[AUDIT QUESTIONS](https://github.com/01-edu/public/tree/master/subjects/rt/audit)
Pictures for audit questions can be found in `solution` folder.

## Usage
### Running the program
```shell
cargo run > [target_file_name].ppm
```
### Configuration
To configure the scene use `/src/config.rs` file.
In Config `new` method you can find all the configurable elements, such as **Image**, **Camera**, **Light** and **Scene**, all following to the corresponding comments.

### Image
Image is responsable for the size and quality of the image. 

``` rust
// Image
let width = 300;
let height = 200;
let samples_per_pixel = 25;
```
So **width** and **height** corresponds to final image witdth and height in pixels. 
**samples_per_pixels** change how many times a ray is sent towards every pixel. This property helps deal with aliasing. So the more the better, but that also means slower.

### Camera
This shouldn't come as suprise but **camera** is responsible for camera properties and placement.
One thing to keep in mind from this point forward, that our coordinate system looks something like [this](https://raytracing.github.io/images/fig-1.03-cam-geom.jpg).
```rust
// Camera
let origin = Point3D::new(0., 2., 0.);
let direction = Point3D::new(0., 0., -4.);
let up = Point3D::new(0., 1., 0.);
let fov = 25.;
```
**Point3D** is representing point or vecor in a 3d scene with 3 coordinates -> x, y, z.
**origin** determines cameras origin point.
**direction** determines which direction camera is facing.
**up** determines how you "hold" the camera. In given scenario where y = 1.0 the camera's up is upwards, if you change y = -1.0, the world will be upside down! And if you really want to lay on your side, change the x coordinate to 1.0.
**fov** is short for field of view. This is a handy little thing  that works similary to focal length/zoom in real camera lenses. In range of 0.0 - 90.0. The larger the number the wider the "lens". Smaller number -> field more narrow and zoomed in.

### Light
To light the scene we use both global illumination and light source. Global ilumination comes as default, so only light source is cnfigurable.
```rust
// Light
let center = Point3D::new(5., 10., -5.);
let intensity = 0.5;
```
Ligt source can be controlled by two properties. **center** determines here the light object is located. **intensity** controlls the brightnes, that ranes form 0.0 to 1.0

### Scene
Scene is a collection of objects that are placed in the scene. Program comes with 3 basic scenes as examples.
Create a scene:
```rust
impl Config {
       fn new_scene() -> Vec<Box<dyn Hittable>>{
              // create object collection
              let mut objects: Vec<Box<dyn Hittable>> = vec![];
              // create object
              //let object = ...
              // wrap object in smart box pointer
              let boxed_obj: Box<dyn Hittable> = Box::new(object);
              // add object into collection
              objects.push(boxed_obj);
              // return the object collection
              objects
       }
}
```
### Objects
As mentioned above there are 4 basic objects provided, so here is how to create them:
#### Plane
```rust
let plane_point = Point3D::new(0., -1., 0.);
let plane_normal = Point3D::new(0., 1., 0.).unit_vector();

let plane = Plane::new(plane_point, plane_normal);
```
Plane consists of two properties: **point** and **normal**
**Point** is any given 3D point in a plane. 
**normal** is a plane's normal vector. 
Plane is created in a default gray color.

#### Sphere
```rust
let center = Point3D::new(0., 0., -6.);
let radius = 1.;
let material = Material::matte(Color::green());

let sphere = Sphere::new(center,radius, material);
```
Sphere can be created based on:
**center** - 3D point
**radius** - as a f64 value
**material** - Check ou options in material section down below.

#### Cube
```rust
let min = Point3D::new(-2., -1., -4.);
let max = Point3D::new(-1., 0., -5.);
let material = Material::matte(Color::red());

let cube = Cube::new(min, max, material);
```
**min**  and **max** are used to define the minimum and maximum corner points of the cube. min is a point that defines the corner of the cube with the smallest x, y and z coordinates, and max is a point that defines the corner of the cube with the largest x, y and z coordinates. And for **material** also check section below. Cube is aligned with x and y axis.


#### Cylinder
```rust
let base = Point3D::new(2., -1. ,-7.);
let radius = 0.5;
let height = 2.;
let material = Material::matte(Color::green());

let cylinder = Cylinder::new(base,r,  material, height);
```
Cylinder is created based on **base** which is a center point of the cylinders base cap.
**radius** and **height** as f64 values. Also **material**.
Cylinders axis is aligned with y axis. So it is always upright.

#### Material
There are two materials provided: matte and metal.
```rust
//metal
let material = Material::metal();
```
For matte surface you can also provide specific color. Some colors are provided by default.
```rust
let material = Material::matte(Color::red());
// more options
// Color::green()
// Color::gray()
// Color::white()
// Color::black()
```
But you can also create any rgb color, by providing rgb values, that are in range of 0.0 to 1.0
```rust
// so if you are going for minty color try this
Color::new(0.3, 1.0, 0.5)
```

 