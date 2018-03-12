extern crate serde;
extern crate serde_json;
extern crate mersh;

#[macro_use]
extern crate rouille;

fn main() {

    let cmds = vec![
        mersh::cmd::In::PushVertex3d([6., 6., 6.]),
        mersh::cmd::In::PushVertex3d([66., 0.00, 6.]),
        mersh::cmd::In::PushVertex3d([0., 0., 0.666]),
        mersh::cmd::In::PushTaggedEdge([0, 0], "MyEdge".to_string()),
        mersh::cmd::In::PushTaggedEdge([0, 1], "MyEdge".to_string()),
        mersh::cmd::In::GetVertex3d(2)
    ];

    // Starting server printing mesh in json format.
    rouille::start_server("localhost:8000", move |request| router!(request,
       (GET) (/) => {

            let mut mesh = mersh::mesh::Mesh3d::default();
            let outputs = mersh::cmd::apply_commands_3d(&mut mesh, &cmds);
            rouille::Response::text(serde_json::to_string(&outputs).unwrap())

       },
       _ => rouille::Response::empty_404()
    ));
}