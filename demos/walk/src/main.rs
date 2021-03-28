use flo_curves::*;
use flo_curves::bezier::*;
use flo_draw::*;
use flo_draw::canvas::*;

fn main() {
    with_2d_graphics(|| {
        let canvas          = create_canvas_window("Bezier curve walking demo");
        let curve_1         = Curve::from_points(Coord2(100.0, 500.0), (Coord2(400.0, 2000.0), Coord2(1500.5, 1200.0)), Coord2(1400.0, 900.0));
        let curve_2         = Curve::from_points(Coord2(100.0, 100.0), (Coord2(400.0, 1600.0), Coord2(1500.5, 800.0)), Coord2(1400.0, 500.0));

        canvas.draw(|gc| {
            gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));

            gc.canvas_height(1500.0);
            gc.center_region(0.0, 0.0, 1500.0, 1500.0);
            
            gc.line_width(2.0);

            gc.new_path();
            gc.move_to(curve_1.start_point().x() as _, curve_1.start_point().y() as _);
            gc.bezier_curve(&curve_1);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.6, 1.0));
            gc.stroke();

            gc.new_path();
            gc.move_to(curve_2.start_point().x() as _, curve_2.start_point().y() as _);
            gc.bezier_curve(&curve_2);
            gc.stroke_color(Color::Rgba(0.0, 0.0, 0.6, 1.0));
            gc.stroke();

            gc.stroke_color(Color::Rgba(1.0, 0.6, 0.0, 1.0));
            gc.fill_color(Color::Rgba(1.0, 0.6, 0.0, 1.0));
            for section in walk_curve_unevenly(&curve_1, 20) {
                let (_t_min, t_max) = section.original_curve_t_values();
                let pos             = curve_1.point_at_pos(t_max);
                let unit_normal     = curve_1.normal_at_pos(t_max).to_unit_vector();

                gc.new_path();
                gc.move_to((pos.x() + unit_normal.x()*12.0) as _, (pos.y() + unit_normal.y()*12.0) as _);
                gc.line_to((pos.x() - unit_normal.x()*12.0) as _, (pos.y() - unit_normal.y()*12.0) as _);
                gc.stroke();

                gc.new_path();
                gc.circle(pos.x() as _, pos.y() as _, 6.0);
                gc.fill();
            }

            for section in walk_curve_evenly(&curve_2, curve_length(&curve_2, 0.1)/20.0, 0.1) {
                let (_t_min, t_max)  = section.original_curve_t_values();
                let pos             = curve_2.point_at_pos(t_max);
                let unit_normal     = curve_2.normal_at_pos(t_max).to_unit_vector();

                gc.new_path();
                gc.move_to((pos.x() + unit_normal.x()*12.0) as _, (pos.y() + unit_normal.y()*12.0) as _);
                gc.line_to((pos.x() - unit_normal.x()*12.0) as _, (pos.y() - unit_normal.y()*12.0) as _);
                gc.stroke();

                gc.new_path();
                gc.circle(pos.x() as _, pos.y() as _, 6.0);
                gc.fill();
            }
        });
    });
}
