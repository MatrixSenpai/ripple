include <BOSL2/std.scad>

$fn = 90;

difference() {
    cube([80, 80, 2]);
    
    right(73) up(3) back(7) yrot(180)
        #import("/home/matrix/Downloads/neopixel-ring-24x-1.snapshot.6/Neopixels_ring_x24.STL");
}