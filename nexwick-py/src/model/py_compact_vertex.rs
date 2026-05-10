use nexwick::model::Vertex;
use pyo3::prelude::*;

/// A vertex (node) in a compact phylogenetic tree.
///
/// Labels are stored as integer indices; resolve them with `LeafLabelMap.get_label`.
#[pyclass(name = "CompactVertex", from_py_object)]
#[derive(Clone)]
pub struct PyCompactVertex {
    inner: Vertex<usize>,
}

impl PyCompactVertex {
    pub fn new(vertex: Vertex<usize>) -> Self {
        PyCompactVertex { inner: vertex }
    }
}

#[pymethods]
impl PyCompactVertex {
    /// Index of this vertex in the tree arena.
    #[getter]
    fn index(&self) -> usize {
        self.inner.index()
    }

    fn is_root(&self) -> bool {
        self.inner.is_root()
    }

    fn is_internal(&self) -> bool {
        self.inner.is_internal()
    }

    fn is_leaf(&self) -> bool {
        self.inner.is_leaf()
    }

    /// Label index for leaf vertices; `None` for internal/root vertices.
    ///
    /// Resolve to a string with `LeafLabelMap.get_label(label_index)`.
    #[getter]
    fn label_index(&self) -> Option<usize> {
        self.inner.label().copied()
    }

    /// Branch length (distance to parent), or `None` for the root or if unset.
    #[getter]
    fn branch_length(&self) -> Option<f64> {
        self.inner.branch_length().map(|bl| *bl)
    }

    /// Indices of the two children for root/internal vertices, or `None` for leaves.
    #[getter]
    fn children(&self) -> Option<(usize, usize)> {
        self.inner.children()
    }

    /// Index of the parent vertex, or `None` for the root.
    #[getter]
    fn parent(&self) -> Option<usize> {
        self.inner.parent()
    }

    fn __repr__(&self) -> String {
        format!("{}", self.inner)
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}
