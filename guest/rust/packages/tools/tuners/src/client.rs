use ambient_api::{
    core::{
        app::components::name, network::components::no_sync, rendering::components::color,
        text::components::font_size,
    },
    prelude::*,
};
use packages::this::{components::*, messages::UpdateTuner};

pub mod packages;

#[main]
pub fn main() {
    Tuners.el().spawn_interactive();

    spawn_query(raw_value()).excludes(no_sync()).bind(|tuners| {
        for (tuner, raw) in tuners {
            entity::add_component(tuner, client_raw(), raw);
        }
    });
    change_query(client_raw())
        .track_change(client_raw())
        .bind(|tuners| {
            for (tuner, raw) in tuners {
                UpdateTuner {
                    id: tuner,
                    raw: raw,
                }
                .send_server_reliable();
            }
        });
}

#[element_component]
fn Tuners(hooks: &mut Hooks) -> Element {
    let tuners = ambient_api::element::use_query(hooks, (tuner_min(), tuner_max(), client_raw()));
    // for (tuner, (tmin, tmax, raw)) in tuners {}

    // let frame_times = use_ref_with(hooks, |_| Vec::new());
    // let rerender = use_rerender_signal(hooks);
    // use_frame(hooks, {
    //     to_owned!(frame_times);
    //     move |_| {
    //         let mut frame_times = frame_times.lock();
    //         frame_times.push(delta_time());
    //         if frame_times.len() > 100 {
    //             frame_times.remove(0);
    //         }
    //         rerender();
    //     }
    // });
    // let fps = {
    //     let frame_times = frame_times.lock();
    //     let fps = frame_times.len() as f32 / frame_times.iter().sum::<f32>();
    //     fps
    // };
    // Text::el(format!("Fps: {fps}"))
    FlowColumn::el(tuners.into_iter().map(|(tuner, (tmin, tmax, raw))| {
        let tname = entity::get_component(tuner, name()).unwrap_or("(Noname Tuner)".to_string());
        let tdesc = entity::get_component(tuner, description()).unwrap();
        let t_int: bool = entity::has_component(tuner, is_int());
        let output_value = tmin + raw * (tmax - tmin);
        let t_off: bool = output_value <= 0. && entity::has_component(tuner, is_nonpositive_off());
        FlowColumn::el([
            Text::el(format!("{}", tname))
                .with(font_size(), 15.)
                .with(color(), Vec3::splat(0.8).extend(1.)),
            Text::el(format!("{}", tdesc)).with(font_size(), 10.),
            FlowRow::el([
                Text::el(format!(" = ")).with(font_size(), 20.),
                Text::el(match (t_int, t_off) {
                    (_, true) => "OFF".to_string(),
                    (true, _) => format!("{:.0}", output_value),
                    _ => format!("{:.2}", output_value),
                })
                .with(font_size(), 20.)
                .with(
                    color(),
                    match t_off {
                        true => vec4(0.8, 0.2, 0.2, 1.0), // if it's OFF, make it red
                        false => Vec4::splat(1.),
                    },
                ),
            ]),
            Slider::new_for_entity_component(hooks, tuner, client_raw()).el(),
        ])
        .with_background(Vec3::splat(1.).extend(0.05))
        .with_margin_even(5.)
        .with_padding_even(5.)
    }))
}
