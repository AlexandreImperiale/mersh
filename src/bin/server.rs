extern crate serde;
extern crate serde_json;
extern crate mersh;

#[macro_use]
extern crate rouille;

fn main() {

    rouille::start_server("localhost:8000", move |request| router!(request,
       (GET) (/) => {

                let mut interpreter = mersh::interpreter::Interpreter::default();
                interpreter.push_cmd(mersh::interpreter::Cmd::NewMesh2d{output_id: "mesh".to_string()});
                interpreter.push_cmd(mersh::interpreter::Cmd::NewVecFloat{input: vec![0., 0.], output_id: "vec".to_string()});

                rouille::Response::text(serde_json::to_string(&interpreter).unwrap())
         },
       _ => rouille::Response::empty_404()
    ));

}