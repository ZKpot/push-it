use super::{ physics, drone, Action };

use rapier3d::{
    na::{
        Vector3,
        geometry::{ UnitQuaternion, },
    },
};

use dotrix::{
    components::{ Model, },
    ecs::{ Mut, Const, },
    services::{ Assets, Camera, World, Input, },
    math::{ Point3, Quat, },
};

use std::f32::consts::PI;

pub fn control(
    world: Mut<World>,
    mut bodies: Mut<physics::BodiesService>,
    input: Const<Input>,
    mut camera: Mut<Camera>,
) {
    // Query player entity
    let query = world.query::<(&mut Model, &mut physics::RigidBody, &mut drone::Stats)>();

    // this loop will run only once, because Player component is assigned to only one entity
    for (model, rigid_body, stats) in query {

        let body = bodies.get_mut(rigid_body.handle).unwrap();
        let postion = body.position().translation;

        //TO DO: rething dw1 and dw2 usage
        let dw1 = UnitQuaternion::from_euler_angles(0.0, 0.0, -PI/2.0);
        let rotation = body.position().rotation * dw1.inverse();

        if stats.is_player {
            let target_xz_angle = camera.xz_angle;
            let target_y_angle = camera.y_angle;

            //TO DO: rething PI/2.0 shift
            let target_rotation = UnitQuaternion::from_euler_angles(
                0.0,
                -target_xz_angle,
                PI/2.0 - target_y_angle
            );

            let delta_rotation = target_rotation * rotation.inverse();
            let delta_axis = match delta_rotation.axis() {
                Some(x) => Vector3::new(
                    x.into_inner().data[0],
                    x.into_inner().data[1],
                    x.into_inner().data[2],
                ),
                None    => Vector3::new(0.0, 0.0, 0.0),
            };

            let delta_angle = delta_rotation.angle();

            let rotation_euler = rotation.euler_angles();

            println!("{:?}", rotation_euler);

            let fwd = Vector3::new(
                -rotation_euler.2.sin() * rotation_euler.1.cos(),
                rotation_euler.1.sin(),
                -rotation_euler.2.cos() * rotation_euler.1.cos(),
            );

            let side = Vector3::new(
                -(-PI/2.0 + rotation_euler.2).sin(),
                0.0,
                -(-PI/2.0 + rotation_euler.2).cos()
            );

            if input.is_action_hold(Action::MoveForward) {
                body.apply_force(fwd * 1.0, true);
            };
            if input.is_action_hold(Action::MoveBackward) {
                body.apply_force(fwd * -1.0, true);
            };
            if input.is_action_hold(Action::MoveLeft) {
                body.apply_force(side * -1.0, true);
            };
            if input.is_action_hold(Action::MoveRight) {
                body.apply_force(side * 1.0, true);
            };

            body.apply_torque(delta_axis * delta_angle * 50.0, true);


            // make camera following the player
            camera.target = Point3::new(postion.x, postion.y, postion.z);
            camera.set_view();
        }

        // apply translation to the model
        model.transform.translate.x = postion.x;
        model.transform.translate.y = postion.y;
        model.transform.translate.z = postion.z;

        //TO DO: rething dw1 and dw2 usage
        let dw2 = UnitQuaternion::from_euler_angles(0.0, 0.0, PI/2.0);
        let rot = rotation * dw2.inverse();

        // apply rotation to the model
        model.transform.rotate = Quat::new(
            rot.into_inner().w,
            rot.into_inner().j,
            rot.into_inner().k,
            rot.into_inner().i,
        );
    }
}

pub fn spawn(
    world: &mut World,
    assets: &mut Assets,
    bodies: &mut physics::BodiesService,
    position: Point3,
) {
    drone::spawn(
        world,
        assets,
        bodies,
        position,
        drone::Stats{ is_player: true },
    );
}