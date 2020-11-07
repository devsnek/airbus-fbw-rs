use ::msfs::{
    msfs,
    msfs::MSFSEvent,
    sim_connect::{data_definition, DataXYZ, Period, SimConnectRecv, SIMCONNECT_OBJECT_ID_USER},
};

#[data_definition]
struct SimData {
    #[name = "G FORCE"]
    #[unit = "GFORCE"]
    g_force: f64,
    #[name = "PLANE PITCH DEGREES"]
    #[unit = "DEGREE"]
    plane_pitch_degrees: f64,
    #[name = "PLANE BANK DEGREES"]
    #[unit = "DEGREE"]
    plane_bank_degrees: f64,
    #[name = "STRUCT WORLD ROTATION VELOCITY"]
    #[unit = "STRUCT"]
    world_rotation_velocity: DataXYZ,
    #[name = "STRUCT BODY ROTATION ACCELERATION"]
    #[unit = "STRUCT"]
    body_rotation_acceleration: DataXYZ,
    #[name = "ELEVATOR TRIM POSITION"]
    #[unit = "DEGREE"]
    elevator_trim: f64,
    #[name = "AIRSPEED INDICATED"]
    #[unit = "KNOTS"]
    indicated_airspeed: f64,
    #[name = "RADIO HEIGHT"]
    #[unit = "FEET"]
    radio_height: f64,
    #[name = "CG PERCENT"]
    #[unit = "PERCENT OVER 100"]
    cg: f64,
    #[name = "FLAPS HANDLE INDEX"]
    #[unit = "NUMBER"]
    flaps_handle_index: f64,
}

#[data_definition]
struct GearPositions {
    #[name = "GEAR ANIMATION POSITION:0"]
    #[unit = "POSITION"]
    gear_animation_position_0: f64,
    #[name = "GEAR ANIMATION POSITION:1"]
    #[unit = "POSITION"]
    gear_animation_position_1: f64,
    #[name = "GEAR ANIMATION POSITION:2"]
    #[unit = "POSITION"]
    gear_animation_position_2: f64,
}

#[data_definition]
struct OutputDataNoTrim {
    #[name = "ELEVATOR POSITION"]
    #[unit = "POSITION"]
    elevator: f64,
    #[name = "AILERON POSITION"]
    #[unit = "POSITION"]
    ailerons: f64,
    #[name = "RUDDER POSITION"]
    #[unit = "POSITION"]
    rudder: f64,
}

#[data_definition]
struct OutputDataTrim {
    #[name = "ELEVATOR TRIM POSITION"]
    #[unit = "DEGREE"]
    elevator_trim: f64,
}

#[msfs::gauge(name=FBW)]
async fn fbw(mut gauge: msfs::Gauge) -> Result<(), Box<dyn std::error::Error>> {
    let mut sim = gauge.open_simconnect("A32NX_FBW")?;

    let mut model = model::FBW::default();

    while let Some(event) = gauge.next_event().await {
        match event {
            MSFSEvent::PanelServiceID(service_id) => match service_id {
                msfs::PanelServiceID::PostInstall => {
                    sim.request_data_on_sim_object::<SimData>(
                        0,
                        SIMCONNECT_OBJECT_ID_USER,
                        Period::SimFrame,
                    )?;
                    sim.request_data_on_sim_object::<GearPositions>(
                        1,
                        SIMCONNECT_OBJECT_ID_USER,
                        Period::SimFrame,
                    )?;

                    sim.map_client_event_to_sim_event(0, "AXIS_ELEVATOR_SET")?;
                    sim.map_client_event_to_sim_event(1, "AXIS_AILERONS_SET")?;
                    sim.map_client_event_to_sim_event(2, "AXIS_RUDDER_SET")?;

                    sim.add_client_event_to_notification_group(0, 0, true)?;
                    sim.add_client_event_to_notification_group(0, 1, true)?;
                    sim.add_client_event_to_notification_group(0, 2, true)?;

                    sim.set_notification_group_priority(
                        0,
                        ::msfs::sys::SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE,
                    )?;
                }
                msfs::PanelServiceID::PreUpdate => {
                    model.step();

                    sim.set_data_on_sim_object(
                        SIMCONNECT_OBJECT_ID_USER,
                        &OutputDataNoTrim {
                            elevator: model.output().sim.raw.output.eta_pos,
                            ailerons: model.output().sim.raw.output.xi_pos,
                            rudder: model.output().sim.raw.output.zeta_pos,
                        },
                    )?;

                    if model.output().sim.raw.output.iH_deg_should_write == 1.0 {
                        sim.set_data_on_sim_object(
                            SIMCONNECT_OBJECT_ID_USER,
                            &OutputDataTrim {
                                elevator_trim: model.output().sim.raw.output.iH_deg,
                            },
                        )?;
                    }
                }
                _ => {}
            },
            MSFSEvent::SimConnect(recv) => match recv {
                SimConnectRecv::Event(event) => {
                    let map = |n| (n / 16384) as f64;
                    match event.uEventID {
                        0 => {
                            model.input().input.delta_eta_pos = map(event.dwData);
                        }
                        1 => {
                            model.input().input.delta_xi_pos = map(event.dwData);
                        }
                        2 => {
                            model.input().input.delta_zeta_pos = map(event.dwData);
                        }
                        _ => {}
                    }
                }
                SimConnectRecv::SimObjectData(event) => {
                    let i = model.input();
                    match event.dwRequestID {
                        0 => {
                            let data = event.into::<SimData>(&sim).unwrap();
                            i.data.nz_g = data.g_force;
                            i.data.Theta_deg = data.plane_pitch_degrees;
                            i.data.Phi_deg = data.plane_bank_degrees;
                            i.data.qk_rad_s = data.world_rotation_velocity.x;
                            i.data.rk_rad_s = data.world_rotation_velocity.y;
                            i.data.pk_rad_s = data.world_rotation_velocity.z;
                            i.data.q_dot_rad_s2 = data.body_rotation_acceleration.x;
                            i.data.r_dot_rad_s2 = data.body_rotation_acceleration.y;
                            i.data.p_dot_rad_s2 = data.body_rotation_acceleration.z;
                            i.data.iH_deg = data.elevator_trim;
                            i.data.Vk_kt = data.indicated_airspeed;
                            i.data.radio_height_ft = data.radio_height;
                            i.data.CG_percent_MAC = data.cg;
                            i.data.flaps_handle_index = data.flaps_handle_index;
                        }
                        1 => {
                            let data = event.into::<GearPositions>(&sim).unwrap();
                            i.data.gear_animation_pos_0 = data.gear_animation_position_0;
                            i.data.gear_animation_pos_1 = data.gear_animation_position_1;
                            i.data.gear_animation_pos_2 = data.gear_animation_position_2;
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
        }
    }

    Ok(())
}
