mod sys {
    #![allow(clippy::all)]
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(safe_packed_borrows)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// Fly-by-wire Model
#[derive(Debug)]
pub struct FBW(sys::FlyByWireModelClass);

impl Default for FBW {
    fn default() -> Self {
        FBW(unsafe { sys::FlyByWireModelClass::new() })
    }
}

impl FBW {
    /// Step the model based on the currently set inputs
    pub fn step(&mut self) {
        unsafe {
            self.0.step();
        }
    }

    /// Access the input data for writing
    pub fn input(&mut self) -> &mut sys::fbw_input {
        &mut self.0.FlyByWire_U.in_
    }

    /// Access the output data for reading
    pub fn output(&mut self) -> &sys::fbw_output {
        &self.0.FlyByWire_Y.out
    }
}

impl Drop for FBW {
    fn drop(&mut self) {
        unsafe {
            self.0.terminate();

            // virtual destructor, can't be linked, but it doesn't do anything anyway.
            // self.0.destruct();
        }
    }
}

#[test]
fn test() {
    let mut fbw = FBW::default();
    {
        let input = fbw.input();
        input.data.nz_g = -1.0;
        input.data.Theta_deg = 1.0;
        input.data.Phi_deg = 24.0;
    }
    fbw.step();
    println!("{:#?}", fbw);
    drop(fbw);
}
