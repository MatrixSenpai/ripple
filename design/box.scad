include <BOSL2/std.scad>
include <BOSL2/walls.scad>
include <temp_display_panel.scad>

$fn = 90;

xrot(90) down(85)
union() {
    // bottom
    cuboid(
        [80, 80, 3],
        edges=[BOTTOM+RIGHT, BOTTOM+LEFT, BOTTOM+FRONT, BOTTOM+BACK],
        anchor=BOTTOM+LEFT+FRONT,
        chamfer=1.5
    );
    
    // left & right walls
    up(2) xcopies(75) right(37.5)
        hex_panel([80, 80, 5], 1.5, 5, orient=RIGHT, anchor=BOTTOM+RIGHT+FRONT);
    
    // front walls
    up(82) xrot(90) yrot(180) zrot(180)
        faceplate(80, 80);
    
    // top wall
    up(82)
        cuboid(
            [80, 80, 3],
            edges=[TOP+RIGHT, TOP+LEFT, TOP+FRONT, TOP+BACK],
            anchor=BOTTOM+LEFT+FRONT,
            chamfer=1.5
    );
}