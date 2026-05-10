use nexwick::model::CompactTree;
use nexwick::newick::NewickStyle;
use pyo3::prelude::*;

use super::py_compact_vertex::PyCompactVertex;
use super::py_label_map::PyLeafLabelMap;

/// A phylogenetic tree using compact (index-based) leaf label storage.
///
/// Leaf labels are stored as integer indices shared with a `LeafLabelMap`.
#[pyclass(name = "CompactTree")]
pub struct PyCompactTree {
    pub inner: CompactTree,
}

impl PyCompactTree {
    pub fn new(inner: CompactTree) -> Self {
        PyCompactTree { inner }
    }
}

#[pymethods]
impl PyCompactTree {
    /// Name of the tree, if present.
    #[getter]
    fn name(&self) -> Option<String> {
        self.inner.name().cloned()
    }

    fn num_leaves(&self) -> usize {
        self.inner.num_leaves()
    }

    fn num_vertices(&self) -> usize {
        self.inner.num_vertices()
    }

    /// Index of the root vertex.
    fn root_index(&self) -> usize {
        self.inner.root_index()
    }

    /// Returns the vertex at the given arena index.
    fn vertex(&self, index: usize) -> PyCompactVertex {
        PyCompactVertex::new(self.inner.vertex(index).clone())
    }

    /// Returns all vertices in pre-order (parents before children).
    fn vertices(&self) -> Vec<PyCompactVertex> {
        self.inner
            .pre_order_iter()
            .map(|v| PyCompactVertex::new(v.clone()))
            .collect()
    }

    /// Height of the tree (root-to-leaf distance, assumes ultrametric).
    fn height(&self) -> f64 {
        self.inner.height()
    }

    fn total_branch_length(&self) -> f64 {
        self.inner.total_branch_length()
    }

    /// Serialises the tree to a Newick string using the given label map.
    fn to_newick(&self, label_map: &PyLeafLabelMap) -> String {
        self.inner
            .to_newick(&NewickStyle::Label, Some(&label_map.inner))
    }

    fn __repr__(&self) -> String {
        match self.inner.name() {
            Some(name) => format!("CompactTree({}, {} leaves)", name, self.inner.num_leaves()),
            None => format!("CompactTree({} leaves)", self.inner.num_leaves()),
        }
    }
}
