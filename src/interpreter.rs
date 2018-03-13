extern crate std;

use super::base::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use super::mesh::*;
use std::vec::*;

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Definition of data structures.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

/// Definition of resources.
#[derive(Serialize, Deserialize)]
pub enum Resource {
    /// Unsigned integer.
    UInt(usize),
    /// Vector of floats.
    VecFloat(Vec<f64>),
    /// Mesh 2d.
    Mesh2d(Box<Mesh2d>),
    /// Mesh3d.
    Mesh3d(Box<Mesh3d>)
}

/// Definition of commands.
#[derive(Serialize, Deserialize)]
pub enum Cmd {
    /// Creating a new unsigned integer.
    NewUInt{ input: usize, output_id: String },
    /// Creating a new vector of floats.
    NewVecFloat{ input: Vec<f64>, output_id: String },
    /// Creating a new 2d mesh.
    NewMesh2d{ output_id: String },
    /// Creating a new 3d mesh.
    NewMesh3d{ output_id: String },
    /// Pushing vertex into 2d mesh.
    PushVertex2d{ mesh_id: String, coords_id: String },
    /// Accessing vertex in 2d mesh.
    GetVertex2d{ mesh_id: String, idx_id: String, output_id: String }
}

/// Definition of an interpreted.
#[derive(Default, Serialize, Deserialize)]
pub struct Interpreter {
    /// Mapping between resource id and resources.
    pub resources: HashMap<String, Resource>,
    /// Associated command history.
    pub cmd_history: VecDeque<Cmd>,
    /// Associated command in queue.
    pub cmd_queue: VecDeque<Cmd>
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Interpreter {
    /// Pushing a command into interpreter queue.
    ///
    /// * `cmd` - a command to push in the queue.
    ///
    /// # Example
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// interpreter.push_cmd(Cmd::NewUInt{ input: 12, output_id: "MyInt".to_string() });
    ///
    /// assert_eq!(interpreter.cmd_queue.len(), 1);
    /// ```
    pub fn push_cmd(&mut self, cmd: Cmd)
    {
        self.cmd_queue.push_back(cmd)
    }

    /// Applying every commands pushed in the interpreter queue.
    /// Commands in queue are automatically moved to interpreter history.
    ///
    /// # Example
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// interpreter.push_cmd(Cmd::NewUInt{ input: 12, output_id: "MyInt".to_string() });
    /// interpreter.apply_cmds_in_queue();
    ///
    /// assert_eq!(interpreter.cmd_history.len(), 1);
    /// assert_eq!(interpreter.cmd_queue.len(), 0);
    /// if interpreter.resources.get(&"MyInt".to_string()).is_none() { assert!(false); }
    ///
    /// ```
    pub fn apply_cmds_in_queue(&mut self)
    {
        for _ in 0..self.cmd_queue.len() {
            let cmd = self.cmd_queue.pop_front().unwrap();
            self.apply_cmd(&cmd);
            self.cmd_history.push_back(cmd);
        }
    }

    /// Applying a command.
    ///
    /// * `cmd` - a command to be applied on resources.
    ///
    /// # Example
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// interpreter.apply_cmd(&Cmd::NewUInt{ input: 12, output_id: "MyInt".to_string() });
    /// if interpreter.resources.get(&"MyInt".to_string()).is_none() { assert!(false); }
    /// ```
    pub fn apply_cmd(&mut self, cmd: &Cmd)
    {
        match *cmd {
            Cmd::NewUInt { input, ref output_id } => self.new_uint(input, output_id),
            Cmd::NewVecFloat { ref input, ref output_id } => self.new_vec_float(input, output_id),
            Cmd::NewMesh2d { ref output_id } => self.new_mesh2d(output_id),
            Cmd::NewMesh3d { ref output_id } => self.new_mesh3d(output_id),
            Cmd::PushVertex2d { ref mesh_id, ref coords_id } => self.push_vertex2d(mesh_id, coords_id),
            _ => panic!("Unsupported command.")
        }
    }

    /// Adding an unsigned integer in resources.
    /// This function panics if output id is already defined.
    ///
    /// * `input` - input unsigned integer to be added to interpreter resources.
    /// * `output_id` - associated (unique) id.
    ///
    /// # Example 0
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyInt");
    ///
    /// interpreter.new_uint(12, &id);
    /// if let Some(rsrc) = interpreter.resources.get(&id) {
    ///     match rsrc {
    ///         &Resource::UInt(uint) => assert_eq!(uint, 12),
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    ///
    /// # Example 1
    /// ```should_panic
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyInt");
    ///
    /// interpreter.new_uint(12, &id);
    /// interpreter.new_uint(223, &id); // => panics here !
    /// ```
    pub fn new_uint(&mut self, input: usize, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::UInt(input));
    }

    /// Adding a vector of floats in resources.
    ///
    /// * `input` - input unsigned integer to be added to interpreter resources.
    /// * `output_id` - associated (unique) id.
    ///
    /// # Example 0
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyId");
    ///
    /// interpreter.new_vec_float(&vec![0., 25.0, 6.0], &id);
    /// if let Some(rsrc) = interpreter.resources.get(&id) {
    ///     match rsrc {
    ///         &Resource::VecFloat(ref vec_float) => assert_eq!(vec_float[1], 25.0),
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    ///
    /// # Example 1
    /// ```should_panic
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyInt");
    ///
    /// interpreter.new_vec_float(&vec![0., 25.0, 6.0], &id);
    /// interpreter.new_vec_float(&vec![0., 9.0], &id); // => panics here !
    /// ```
    pub fn new_vec_float(&mut self, input: &[f64], output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::VecFloat(input.to_vec()));
    }

    /// Adding a new 2d mesh in resources.
    ///
    /// * `output_id` - associated (unique) id of the 2d mesh.
    ///
    /// # Example 0
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyMesh");
    ///
    /// interpreter.new_mesh2d(&id);
    /// if let Some(rsrc) = interpreter.resources.get(&id) {
    ///     match rsrc {
    ///         &Resource::Mesh2d(_) => assert!(true),
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    ///
    /// # Example 1
    /// ```should_panic
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyMesh");
    ///
    /// interpreter.new_mesh2d(&id);
    /// interpreter.new_mesh2d(&id); // => panics here !
    /// ```
    pub fn new_mesh2d(&mut self, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::Mesh2d(Box::new(Mesh2d::default())));
    }

    /// Adding a new 3d mesh in resources.
    ///
    /// * `output_id` - associated (unique) id of the 3d mesh.
    ///
    /// # Example 0
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyMesh");
    ///
    /// interpreter.new_mesh3d(&id);
    /// if let Some(rsrc) = interpreter.resources.get(&id) {
    ///     match rsrc {
    ///         &Resource::Mesh3d(_) => assert!(true),
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    ///
    /// # Example 1
    /// ```should_panic
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    /// let id = String::from("MyMesh");
    ///
    /// interpreter.new_mesh3d(&id);
    /// interpreter.new_mesh3d(&id); // => panics here !
    /// ```
    pub fn new_mesh3d(&mut self, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::Mesh3d(Box::new(Mesh3d::default())));
    }

    /// Pushing a 2d vertex into a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `coords_id` - id associated to the coordinates of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `coords_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    /// # Example
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    ///
    /// let coords_id = String::from("MyCoords");
    /// interpreter.new_vec_float(&vec![0.0, 1.0], &coords_id);
    ///
    /// let mesh_id = String::from("MyMesh");
    /// interpreter.new_mesh2d(&mesh_id);
    ///
    /// interpreter.push_vertex2d(&mesh_id, &coords_id);
    ///
    /// if let Some(rsrc) = interpreter.resources.get(&mesh_id) {
    ///     match rsrc {
    ///         &Resource::Mesh2d(ref mesh) => {
    ///                 assert_eq!(mesh.vertices[0].coords.x, 0.0);
    ///                 assert_eq!(mesh.vertices[0].coords.y, 1.0);
    ///             },
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    pub fn push_vertex2d(&mut self, mesh_id: &str, coords_id: &str)
    {
        let mut pnt = Pnt2d::default();
        {
            let vec = self.get_mut_vec_float(coords_id);
            pnt.coords.x = vec[0]; pnt.coords.y = vec[1];
        }
        self.get_mut_mesh2d(mesh_id).vertices.push(pnt);
    }

    /// Accessing a 2d vertex in a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `idx_id` - id associated to the index of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `idx_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    /// # Example
    /// ```
    /// use mersh::interpreter::*;
    ///
    /// let mut interpreter = Interpreter::default();
    ///
    /// let mesh_id = String::from("MyMesh");
    /// interpreter.new_mesh2d(&mesh_id);
    ///
    /// let coords_id = String::from("MyCoords");
    /// interpreter.new_vec_float(&vec![6.0, 1.0], &coords_id);
    ///
    /// interpreter.push_vertex2d(&mesh_id, &coords_id);
    ///
    /// let idx_id = String::from("MyId");
    /// interpreter.new_uint(0, &idx_id);
    ///
    /// let output_id = String::from("MyVertex");
    /// interpreter.get_vertex2d(&mesh_id, &idx_id, &output_id);
    ///
    /// if let Some(rsrc) = interpreter.resources.get(&output_id) {
    ///     match rsrc {
    ///         &Resource::VecFloat(ref vec) => {
    ///                 assert_eq!(vec[0], 6.0);
    ///                 assert_eq!(vec[1], 1.0);
    ///             },
    ///         _ => assert!(false)
    ///     }
    /// } else { assert!(false) }
    /// ```
    pub fn get_vertex2d(&mut self, mesh_id: &str, idx_id: &str, output_id: &str)
    {
        let coords;
        {
            let idx = self.get_mut_uint(idx_id);
            let pnt = &self.get_mut_mesh2d(mesh_id).vertices[idx];
            coords = [pnt.coords.x, pnt.coords.y];
        }
        self.new_vec_float(&coords, output_id);
    }
}

//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////
// private Implementations.
//////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////

impl Interpreter {
    // Checking if id is already defined. This function panics if id is already defined
    //  in the interpreter resources.
    fn panic_if_defined(&self, id: &str)
    {
        if self.resources.get(id).is_some() { panic!("Id already defined.") }
    }

    // Accessing mutable reference to a unsigned integer from its id.
    // This function panics if vector id is not defined.
    fn get_mut_uint(&mut self, uint_id: &str) -> usize
    {
        if let Some(uint_rsrc) = self.resources.get_mut(uint_id) {
            match *uint_rsrc {
                Resource::UInt(uint) => { uint },
                _ => panic!("No unsigned integer associated to id.")
            }
        } else { panic!("Undefined id.") }
    }

    // Accessing mutable reference to a vector of floats from its id.
    // This function panics if vector id is not defined.
    fn get_mut_vec_float(&mut self, vec_id: &str) -> &mut [f64]
    {
        if let Some(vec_rsrc) = self.resources.get_mut(vec_id) {
            match *vec_rsrc {
                Resource::VecFloat(ref mut vec) => { vec },
                _ => panic!("No vector of floats associated to id.")
            }
        } else { panic!("Undefined id.") }
    }

    // Accessing mutable reference to a 2d mesh from its id.
    // This function panics if mesh id is not defined.
    fn get_mut_mesh2d(&mut self, mesh_id: &str) -> &mut Mesh2d
    {
        if let Some(mesh_rsrc) = self.resources.get_mut(mesh_id) {
            match *mesh_rsrc {
                Resource::Mesh2d(ref mut mesh) => { mesh },
                _ => panic!("No 2d mesh associated to id.")
            }
        } else { panic!("Undefined id.") }
    }
}