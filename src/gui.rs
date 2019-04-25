use crate::gen::Generator;
use conrod_core::{position::{Align, Direction, Padding, Relative},
                  *};
use parking_lot::Mutex;
use std::sync::Arc;

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    conrod_core::Theme {
        name:                   "Demo Theme".to_string(),
        padding:                Padding::none(),
        x_position:             Position::Relative(Relative::Align(Align::Start), None),
        y_position:             Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color:       conrod_core::color::rgb(0.24, 0.24, 0.26),
        shape_color:            conrod_core::color::rgb(0.17, 0.17, 0.19),
        border_color:           conrod_core::color::rgb(0.2, 0.2, 0.22),
        border_width:           0.0,
        label_color:            conrod_core::color::rgb(0.78, 0.78, 0.80),
        font_id:                None,
        font_size_large:        26,
        font_size_medium:       18,
        font_size_small:        12,
        widget_styling:         conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold:   0.0,
        double_click_threshold: std::time::Duration::from_millis(400),
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        introduction,
        canvas_scrollbar,
        engine_rpm_slider,
               engine_master_volume_slider,
        engine_intake_volume_slider,
        engine_exhaust_volume_slider,
        engine_engine_vibrations_volume_slider,
    }
}

/// Instantiate a GUI demonstrating every widget available in conrod.
pub fn gui(ui: &mut conrod_core::UiCell, ids: &Ids, generator: Arc<Mutex<Generator>>) {
    let generator = generator.lock();

    const PAD: conrod_core::Scalar = 30.0;

    widget::Canvas::new().pad(PAD).scroll_kids_vertically().set(ids.canvas, ui);

    widget::Text::new("Engine Sound Generator").font_size(24).top_left().set(ids.title, ui);

    {
        let prev_val = generator.get_rpm();
        for value in widget::Slider::new(prev_val, 700.0, 9000.0)
            .label(format!("Engine RPM {:.2}", prev_val).as_str())
            .label_font_size(12)
            .padded_w_of(ids.canvas, PAD)
            .down(5.0)
            .set(ids.engine_rpm_slider, ui)
        {
            generator.set_rpm(value);
        }
    }

    ///////////////////
    // Volumes       //
    ///////////////////

    {
        {
            let prev_val = generator.get_volume();
            for value in widget::Slider::new(prev_val, 0.0, 1.0)
                .label(format!("Master volume {:.0}%", prev_val * 100.0).as_str())
                .label_font_size(12)
                .padded_w_of(ids.canvas, PAD)
                .down(5.0)
                .set(ids.engine_master_volume_slider, ui)
            {
                generator.set_volume(value);
            }
        }

        {
            let prev_val = generator.get_intake_volume();
            for value in widget::Slider::new(prev_val, 0.0, 1.0)
                .label(format!("Intake volume {:.0}%", prev_val * 100.0).as_str())
                .label_font_size(12)
                .padded_w_of(ids.canvas, PAD)
                .down(5.0)
                .set(ids.engine_intake_volume_slider, ui)
            {
                let mut dif = value - prev_val;
                generator.set_intake_volume(value);
                let v1 = generator.get_exhaust_volume();
                let v2 = generator.get_engine_vibrations_volume();
                if v1 < v2 {
                    let vv1 = v1.min(dif * 0.5);
                    dif -= vv1;
                    generator.set_exhaust_volume((v1 - vv1).min(1.0).max(0.0));
                    generator.set_engine_vibrations_volume((v2 - dif).min(1.0).max(0.0));
                } else {
                    let vv2 = v2.min(dif * 0.5);
                    dif -= vv2;
                    generator.set_engine_vibrations_volume((v2 - vv2).min(1.0).max(0.0));
                    generator.set_exhaust_volume((v1 - dif).min(1.0).max(0.0));
                }
            }
        }

        {
            let prev_val = generator.get_exhaust_volume();
            for value in widget::Slider::new(prev_val, 0.0, 1.0)
                .label(format!("Exhaust volume {:.0}%", prev_val * 100.0).as_str())
                .label_font_size(12)
                .padded_w_of(ids.canvas, PAD)
                .down(5.0)
                .set(ids.engine_exhaust_volume_slider, ui)
            {
                let mut dif = value - prev_val;
                generator.set_exhaust_volume(value);
                let v1 = generator.get_intake_volume();
                let v2 = generator.get_engine_vibrations_volume();
                if v1 < v2 {
                    let vv1 = v1.min(dif * 0.5);
                    dif -= vv1;
                    generator.set_intake_volume((v1 - vv1).min(1.0).max(0.0));
                    generator.set_engine_vibrations_volume((v2 - dif).min(1.0).max(0.0));
                } else {
                    let vv2 = v2.min(dif * 0.5);
                    dif -= vv2;
                    generator.set_engine_vibrations_volume((v2 - vv2).min(1.0).max(0.0));
                    generator.set_intake_volume((v1 - dif).min(1.0).max(0.0));
                }
            }
        }

        {
            let prev_val = generator.get_engine_vibrations_volume();
            for value in widget::Slider::new(prev_val, 0.0, 1.0)
                .label(format!("Engine vibrations volume {:.0}%", prev_val * 100.0).as_str())
                .label_font_size(12)
                .padded_w_of(ids.canvas, PAD)
                .down(5.0)
                .set(ids.engine_engine_vibrations_volume_slider, ui)
            {
                let mut dif = value - prev_val;
                generator.set_engine_vibrations_volume(value);
                let v1 = generator.get_exhaust_volume();
                let v2 = generator.get_intake_volume();
                if v1 < v2 {
                    let vv1 = v1.min(dif * 0.5);
                    dif -= vv1;
                    generator.set_exhaust_volume((v1 - vv1).min(1.0).max(0.0));
                    generator.set_intake_volume((v2 - dif).min(1.0).max(0.0));
                } else {
                    let vv2 = v2.min(dif * 0.5);
                    dif -= vv2;
                    generator.set_intake_volume((v2 - vv2).min(1.0).max(0.0));
                    generator.set_exhaust_volume((v1 - dif).min(1.0).max(0.0));
                }
            }
        }

        // normalize again to mitigate any floating point error
        {
            let iv = generator.get_intake_volume();
            let ev = generator.get_exhaust_volume();
            let evv = generator.get_engine_vibrations_volume();
            let sum = iv + ev + evv;
            generator.set_intake_volume(iv / sum);
            generator.set_exhaust_volume(ev / sum);
            generator.set_engine_vibrations_volume(evv / sum);
        }
    }

    widget::Scrollbar::y_axis(ids.canvas).auto_hide(false).set(ids.canvas_scrollbar, ui);
}