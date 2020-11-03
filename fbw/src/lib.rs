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
    #[name = "AIRSPEED INDICATED"]
    #[unit = "KNOTS"]
    indicated_airspeed: f64,
    #[name = "RADIO HEIGHT"]
    #[unit = "FEET"]
    radio_height: f64,
    #[name = "CG PERCENT"]
    #[unit = "PERCENT OVER 100"]
    cg: f64,
    #[name = "STRUCT WORLD ROTATION VELOCITY"]
    #[unit = "STRUCT"]
    world_rotation_velocity: DataXYZ,
}

#[data_definition]
struct OutputData {
    #[name = "ELEVATOR POSITION"]
    #[unit = "POSITION"]
    elevator: f64,
    #[name = "ELEVATOR TRIM POSITION"]
    #[unit = "DEGREE"]
    elevator_trim: f64,
    #[name = "AILERON POSITION"]
    #[unit = "POSITION"]
    ailerons: f64,
}

#[msfs::gauge(name=FBW)]
async fn fbw(mut gauge: msfs::Gauge) -> Result<(), Box<dyn std::error::Error>> {
    let mut sim = gauge.open_simconnect("A32NX_FBW")?;

    let mut model = model::FBW::default();

    while let Some(event) = gauge.next_event().await {
        match event {
            MSFSEvent::PanelServiceID(service_id) => match service_id {
                msfs::PanelServiceID::PostInstall => {
                    sim.add_data_definition::<SimData>()?;
                    sim.add_data_definition::<OutputData>()?;

                    sim.request_data_on_sim_object::<SimData>(
                        0,
                        SIMCONNECT_OBJECT_ID_USER,
                        Period::SimFrame,
                    )?;

                    sim.map_client_event_to_sim_event(0, "AXIS_ELEVATOR_SET")?;
                    sim.map_client_event_to_sim_event(1, "AXIS_AILERONS_SET")?;

                    sim.add_client_event_to_notification_group(0, 0, true)?;
                    sim.add_client_event_to_notification_group(0, 1, true)?;

                    sim.set_notification_group_priority(
                        0,
                        ::msfs::sys::SIMCONNECT_GROUP_PRIORITY_HIGHEST_MASKABLE,
                    )?;
                }
                msfs::PanelServiceID::PreUpdate => {
                    model.step();

                    let output = OutputData {
                        elevator: model.output().sim.raw.output.eta_pos,
                        elevator_trim: model.output().sim.raw.output.iH_deg,
                        ailerons: model.output().sim.raw.output.xi_pos,
                    };

                    sim.set_data_on_sim_object(SIMCONNECT_OBJECT_ID_USER, &output)?;
                }
                _ => {}
            },
            MSFSEvent::SimConnect(recv) => match recv {
                SimConnectRecv::Event(event) => {
                    let map = |n| (n / 13484) as f64;
                    match event.uEventID {
                        0 => {
                            model.input().input.delta_eta_pos = map(event.dwData);
                        }
                        1 => {
                            model.input().input.delta_xi_pos = map(event.dwData);
                        }
                        _ => {}
                    }
                }
                SimConnectRecv::SimObjectData(event) => {
                    let data = event.into::<SimData>(&sim).unwrap();
                    let i = model.input();
                    i.data.nz_g = data.g_force;
                    i.data.Theta_deg = data.plane_pitch_degrees;
                    i.data.Phi_deg = data.plane_bank_degrees;
                    i.data.Vk_kt = data.indicated_airspeed;
                    i.data.radio_height_ft = data.radio_height;
                    i.data.CG_percent_MAC = data.cg;
                    i.data.qk_rad_s = data.world_rotation_velocity.x;
                    i.data.rk_rad_s = data.world_rotation_velocity.y;
                    i.data.pk_rad_s = data.world_rotation_velocity.z;
                }
                _ => {}
            },
        }
    }

    Ok(())
}
