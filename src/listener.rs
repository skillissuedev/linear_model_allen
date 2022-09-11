use crate::{check_al_error, sys::*, Float3, Orientation, PropertiesContainer};

#[non_exhaustive]
pub struct Listener;

impl PropertiesContainer<f32> for Listener {
    fn get(&self, param: i32) -> f32 {
        let result = unsafe {
            let mut value = 0.0;
            alGetListenerf(param, &mut value);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn set(&self, param: i32, value: f32) {
        unsafe { alListenerf(param, value) };
        check_al_error().unwrap();
    }
}

impl PropertiesContainer<[f32; 3]> for Listener {
    fn get(&self, param: i32) -> [f32; 3] {
        let result = unsafe {
            let mut value = [0.0, 0.0, 0.0];
            alGetListener3f(param, &mut value[0], &mut value[1], &mut value[2]);
            value
        };

        check_al_error().unwrap();

        result
    }

    fn set(&self, param: i32, value: [f32; 3]) {
        unsafe { alListener3f(param, value[0], value[1], value[2]) };
        check_al_error().unwrap()
    }
}

impl PropertiesContainer<Orientation> for Listener {
    fn get(&self, param: i32) -> Orientation {
        let mut value = Orientation::default();
        unsafe { alGetListenerfv(param, &mut value as *mut Orientation as *mut f32) };
        value
    }

    fn set(&self, param: i32, value: Orientation) {
        unsafe { alListenerfv(param, &value as *const Orientation as *const f32) };
    }
}

impl Listener {
    getter_setter!(gain, set_gain, f32, AL_GAIN);

    getter_setter!(position, set_position, Float3, AL_POSITION);
    getter_setter!(velocity, set_velocity, Float3, AL_VELOCITY);
    getter_setter!(orientation, set_orientation, Orientation, AL_ORIENTATION);
}
