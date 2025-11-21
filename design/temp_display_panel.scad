include <BOSL2/std.scad>

/* [Faceplate Dimensions] */
// Faceplate Height
FP_HEIGHT = 2;
// Faceplate Width
FP_WIDTH = 65;
// Faceplate Depth
FP_DEPTH = 65;

/* [Display Cutout Parameters] */
// Show Display cutout
DP_ENABLE = true;
// Display studs height
DP_STUD_HEIGHT = 5;
// Display studs diameter
DP_STUD_DIAMETER = 2;
// Display cutout width
DP_WIDTH = 35.5;
// Display cutout height
DP_HEIGHT = 23.5;

/* [LED Cutout Parameters] */
// Show LED cutout
LED_ENABLE = true;
// LED cutout item width
LED_WIDTH = 5.5;
// LED cutout item height
LED_HEIGHT = 5.5;
// LED count
LED_COUNT = 24;
// LED ring diameter
LED_DIAMETER = 58;

module faceplate(
    faceplate_width  = FP_WIDTH,
    faceplate_depth  = FP_DEPTH,
    faceplate_height = FP_HEIGHT,
) {
    assert(faceplate_width >= 60 || faceplate_width == FP_WIDTH, "Faceplace width must be at least 60mm");
    assert(faceplate_height >= 60 || faceplate_height == FP_HEIGHT, "Faceplace height must be at least 60mm");
    assert(faceplate_depth >= 2 || faceplate_depth == FP_DEPTH, "Faceplace depth must be at least 2mm");

    difference() {
        union() {
            cube([faceplate_width, faceplate_depth, faceplate_height]);
        
            right(faceplate_width / 2) back(faceplate_depth / 1.4) up(faceplate_height - 0.01) xcopies(n=2, spacing=31)
                cylinder(h=DP_STUD_HEIGHT, d=DP_STUD_DIAMETER, anchor=BOTTOM+CENTER);
        
            right(faceplate_width / 2) back(faceplate_depth / 3.5) up(faceplate_height - 0.5) xcopies(n=2, spacing=31) xrot(15)
                cylinder(h=DP_STUD_HEIGHT, d=DP_STUD_DIAMETER, anchor=BOTTOM+CENTER);
        }
    
        right(faceplate_width / 2) back(faceplate_depth / 2) down(0.1)
            cube([DP_WIDTH, DP_HEIGHT, faceplate_height + 1], anchor=BOTTOM+CENTER);
    
        right(faceplate_width / 2) back(faceplate_depth / 2) down(0.1) zrot_copies(d=LED_DIAMETER, n=LED_COUNT)
            cube([LED_WIDTH, LED_HEIGHT, faceplate_height + 1], anchor=BOTTOM+CENTER);
    }
};