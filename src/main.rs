/* Ray tracer in one day.
 * Inspired by these blog post: 
 *  - https://raytracing.github.io/books/RayTracingInOneWeekend.html
 *  - https://blog.singleton.io/posts/2022-01-02-raytracing-with-rust/
 */
use std::io::{Write, stderr, stdout, BufWriter};
use rtrace::vec3::{Vec3, Point3, Color};
use rtrace::color::write_color;
use rtrace::ray::*;
use rtrace::raytracer::*;

/*
    - The pixels are written out in rows with pixels left to right.
    - The rows are written out from top to bottom.
    - By convention, each of the red/green/blue components range from 0.0 to 1.0. We will relax that later when we internally use high dynamic range, but before output we will tone map to the zero to one range, so this code won’t change.
    - Red goes from fully off (black) to fully on (bright red) from left to right, and green goes from black at the bottom to fully on at the top. Red and green together make yellow so we should expect the upper right corner to be yellow.
*/
fn basic_image() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = image_width / aspect_ratio as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - 
        vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    for j in (0..image_height-1).rev() {
        eprint!("\rScanlines remaining: {j} ");
        stderr().flush().unwrap();
        for i in 0..image_width {
            let u: f64 = f64::from(i) / f64::from(image_width-1);
            let v: f64 = f64::from(j) / f64::from(image_height-1);

            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pxl: Color = ray_color(&r);
            let mut stdout = BufWriter::new(stdout().lock());
            write_color(&mut stdout, pxl).unwrap();
        }
    }
    eprint!("\nDone.\n");
}

fn test_prints() {
    let v1 = Point3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(69.0, 420.0, 3.0);
    
    let neg_v = -v1;
    println!("Negative vec: {:?}", neg_v);

    let add_v = v1+v2;
    println!("Add vec: {:?}", add_v);

    let sub_v = v1-v2;
    println!("Add vec: {:?}", sub_v);

    let mul_v = v1*v2;
    println!("Add vec: {:?}", mul_v);

    let div_v = v1/v2;
    println!("Add vec: {:?}", div_v);

    println!("Length squared: {}", v1.length_squared());
    println!("Length: {}", v1.length());

    println!("FMT: {}", v1);

    println!("Dot product: {}", v1.dot(&v2));
    println!("Cross product: {}", v1.cross(&v2));

    println!("Unit vector: {}", v1.unit_vector());

    let mut stdout = BufWriter::new(stdout().lock());
    write_color(&mut stdout, v1);

}

fn main() {
    // test_prints();
    // basic_image();
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(69.0, 420.0, 3.0);
    let ray: Ray = Ray::new(v1, v2);
    let sunshine = ray_color(&ray);
    // println!("SCALAR: {}", v1*5.0);
    // println!("{}", sunshine);
    basic_image()
}
