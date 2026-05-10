use nexwick::model::leaf_label_map::LeafLabelMap;
use pyo3::prelude::*;

/// A bidirectional map between leaf label strings and their integer indices.
///
/// Shared across all trees parsed from the same file.
#[pyclass(name = "LeafLabelMap")]
pub struct PyLeafLabelMap {
    pub inner: LeafLabelMap,
}

impl PyLeafLabelMap {
    pub fn new(inner: LeafLabelMap) -> Self {
        PyLeafLabelMap { inner }
    }
}

#[pymethods]
impl PyLeafLabelMap {
    /// Returns the number of distinct labels.
    fn num_labels(&self) -> usize {
        self.inner.num_labels()
    }

    /// Returns the label string for the given index, or `None` if out of range.
    fn get_label(&self, index: usize) -> Option<String> {
        self.inner.get_label(index).map(str::to_owned)
    }

    /// Returns the index for the given label string, or `None` if not present.
    fn get_index(&self, label: &str) -> Option<usize> {
        self.inner.get_index(label)
    }

    /// Returns all labels in index order.
    fn labels(&self) -> Vec<String> {
        self.inner.labels().clone()
    }

    fn __repr__(&self) -> String {
        format!("LeafLabelMap({} labels)", self.inner.num_labels())
    }
}
