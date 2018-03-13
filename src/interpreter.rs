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
    /// Creating a new 3d mesh.
    NewMesh3d{ output_id: String },
    /// Pushing vertex into 3d mesh.
    PushVertex3d{ mesh_id: String, coords_id: String },
    /// Accessing vertex in 3d mesh.
    GetVertex3d{ mesh_id: String, idx_id: String, output_id: String }
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
    pub fn push_cmd(&mut self, cmd: Cmd)
    {
        self.cmd_queue.push_back(cmd)
    }

    /// Applying every commands pushed in the interpreter queue.
    /// Commands in queue are automatically moved to interpreter history.
    ///
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
    pub fn apply_cmd(&mut self, cmd: &Cmd)
    {
        match *cmd {
            Cmd::NewUInt { input, ref output_id } => self.new_uint(input, output_id),
            Cmd::NewVecFloat { ref input, ref output_id } => self.new_vec_float(input, output_id),
            Cmd::NewMesh3d { ref output_id } => self.new_mesh3d(output_id),
            Cmd::PushVertex3d { ref mesh_id, ref coords_id } => self.push_vertex3d(mesh_id, coords_id),
            Cmd::GetVertex3d { ref mesh_id, ref idx_id, ref output_id} => self.get_vertex3d(mesh_id, idx_id, output_id),
        }
    }

    /// Adding an unsigned integer in resources.
    /// This function panics if output id is already defined.
    ///
    /// * `input` - input unsigned integer to be added to interpreter resources.
    /// * `output_id` - associated (unique) id.
    ///
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
    pub fn new_vec_float(&mut self, input: &[f64], output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::VecFloat(input.to_vec()));
    }

    /// Adding a new 3d mesh in resources.
    ///
    /// * `output_id` - associated (unique) id of the 3d mesh.
    ///
    pub fn new_mesh3d(&mut self, output_id: &str)
    {
        self.panic_if_defined(output_id);
        self.resources.insert(String::from(output_id), Resource::Mesh3d(Box::new(Mesh3d::default())));
    }

    /// Pushing a 3d vertex into a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `coords_id` - id associated to the coordinates of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `coords_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    pub fn push_vertex3d(&mut self, mesh_id: &str, coords_id: &str)
    {
        let pnt;
        {
            let vec = self.get_mut_vec_float(coords_id);
            pnt = Pnt3d::new([vec[0], vec[1], vec[2]]);
        }
        self.get_mut_mesh3d(mesh_id).vertices.push(pnt);
    }

    /// Accessing a 3d vertex in a mesh.
    ///
    /// * `mesh_id` - id associated to the mesh.
    /// * `idx_id` - id associated to the index of the vertex.
    ///
    /// This function panics if neither the `mesh_id` nor the `idx_id` are ids associated to
    /// actual resources in the interpreter.
    ///
    pub fn get_vertex3d(&mut self, mesh_id: &str, idx_id: &str, output_id: &str)
    {
        let coords;
        {
            let idx = self.get_mut_uint(idx_id);
            let pnt = &self.get_mut_mesh3d(mesh_id).vertices[idx];
            coords = [pnt.coords.x, pnt.coords.y, pnt.coords.z];
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

    // Accessing mutable reference to a 3d mesh from its id.
    // This function panics if mesh id is not defined.
    fn get_mut_mesh3d(&mut self, mesh_id: &str) -> &mut Mesh3d
    {
        if let Some(mesh_rsrc) = self.resources.get_mut(mesh_id) {
            match *mesh_rsrc {
                Resource::Mesh3d(ref mut mesh) => { mesh },
                _ => panic!("No 3d mesh associated to id.")
            }
        } else { panic!("Undefined id.") }
    }
}