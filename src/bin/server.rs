extern crate serde;
extern crate serde_json;
extern crate mersh;

#[macro_use]
extern crate rouille;

fn main() {

    // Building dummy mesh.
    let mut mesh = mersh::mesh::Mesh3d::default();
    mesh.vertices.push(mersh::base::Pnt3d::new([6., 6., 6.]));
    mesh.vertices.push(mersh::base::Pnt3d::new([66., 0.00, 6.]));
    mesh.vertices.push(mersh::base::Pnt3d::new([0., 0., 0.666]));

    // Starting server printing mesh in json format.
    rouille::start_server("localhost:8000", move |request| {
        router!(request,
           (GET) (/) => { rouille::Response::html(FORM) },
           (POST) (/) => { rouille::Response::html(FORM) },
           _ => rouille::Response::empty_404()
        )
    });
}

// The HTML document of the home page.
static FORM: &'static str = r#"
<html>
    <head>
        <title>Form</title>
    </head>
    <body>
        <form action="" method="POST" enctype="multipart/form-data">
            <p><input type="text" name="txt" placeholder="Some text" /></p>
            <p><input type="file" name="files" multiple /></p>
            <p><button>Upload</button></p>
        </form>
    </body>
</html>
"#;