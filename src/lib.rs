use lotus_script::{Script, script, time::game_time, var::set_var};

script!(MyScript);

#[derive(Default)]
pub struct MyScript {}

impl Script for MyScript {
    fn tick(&mut self) {
        set_var(
            "hand_hour",
            (game_time().primitive_date_time().hour() as f32 / 12.0)
                + (game_time().primitive_date_time().minute() as f32 / 600.0),
        );

        set_var(
            "hand_minute",
            game_time().primitive_date_time().minute() as f32 / 60.0,
        );
    }
}
