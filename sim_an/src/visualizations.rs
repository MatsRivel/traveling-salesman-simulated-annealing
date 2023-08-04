use std::{f32::consts::TAU, ops::ControlFlow, collections::HashMap, io::BufRead, error::Error};

use itertools::Itertools as _;

use rerun::{
    components::{ColorRGBA, LineStrip3D, LineStrip2D, Point3D,Point2D, Radius, Transform3D, Vec3D, Vec2D},
    demo_util::{bounce_lerp, color_spiral},
    external::{glam, re_log_types::hash},
    time::{Time, TimeType, Timeline},
    transform, MsgSender, RecordingStreamBuilder, RecordingStream,
};

pub fn coords_to_points(coords:&HashMap<i32,[i32;2]>) -> HashMap<i32,Point2D>{
    coords
        .iter()
        .map( |(node_number, coord)| {
            let (x,y) = (coord[0] as f32, coord[1] as f32);
            (*node_number, Point2D::new(x,y))
        }).collect::<HashMap<i32, Point2D>>()
    
}
pub fn lines_between_points(points:&HashMap<i32,Point2D>) -> HashMap<(i32,i32),LineStrip2D>{
    /* Makes lines from point a to b for all a in [0..(n-1)], and b in [1..n]
    Returns hashmap with key = (from_node, to_node) and value = LineStrip2D*/
    points
        .iter()
        .zip(points
                .iter()
                .skip(1)
            )
        .map(|((from_node, point_a),(to_node,point_b))|{
            let vector = vec![Vec2D([point_a.x, point_a.y]), Vec2D([point_b.x, point_b.y]) ];
            let key = (*from_node,*to_node);
            (key, LineStrip2D(vector))
         }).collect::<HashMap<(i32, i32), LineStrip2D>>()
}

enum DummyError{
    Smile,
}
pub fn make_visualization(coords:&HashMap<i32,[i32;2]>) -> Result<(),DummyError>{
    // Get the elements to plot:
    let node_points = coords_to_points(coords);
    let lines = lines_between_points(&node_points);
    let node_points_vec = node_points.iter().map(|(key,val)| val).collect();
    // Prepare recording:
    let flush_timeout: Option<std::time::Duration> = Some(std::time::Duration::from_secs(60));
    let recording = RecordingStreamBuilder::new("minimal").connect(rerun::default_server_addr(),flush_timeout)?;

    MsgSender::new("sim_an/points")
                                        .with_component(&node_points_vec)?
                                        .with_splat(ColorRGBA::from([128, 128, 128, 255]))?
                                        .with_timeless(true)
                                        .send(&recording)?;
    
    
    Ok(())

}