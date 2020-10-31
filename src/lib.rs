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
pub struct FBW(sys::fbwModelClass);

impl Default for FBW {
    fn default() -> Self {
        FBW(unsafe { sys::fbwModelClass::new() })
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
    pub fn input(&mut self) -> &mut sys::ExternalInputs_fbw_T {
        &mut self.0.fbw_U
    }

    /// Access the output data for reading
    pub fn output(&mut self) -> &sys::ExternalOutputs_fbw_T {
        &self.0.fbw_Y
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
        input.in_sim_simrawdata_nz_g = -1.0;
        input.in_sim_simrawdata_Theta_deg = 1.0;
        input.in_sim_simrawdata_Phi_deg = 24.0;
    }
    fbw.step();
    println!("{:#?}", fbw);
    drop(fbw);
}
