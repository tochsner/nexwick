use std::path::PathBuf;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::model::py_compact_tree::PyCompactTree;
use crate::model::py_label_map::PyLeafLabelMap;

/// Parses a NEXUS file and returns a list of trees with their shared label map.
///
/// # Arguments
/// * `path` - Path to the NEXUS file (str or pathlib.Path)
///
/// # Returns
/// A tuple `(trees, label_map)` where `trees` is a list of `CompactTree` objects
/// and `label_map` is a `LeafLabelMap` shared across all trees.
///
/// # Raises
/// `ValueError` if the file cannot be read or parsed.
#[pyfunction]
pub fn parse_nexus_file(path: PathBuf) -> PyResult<(Vec<PyCompactTree>, PyLeafLabelMap)> {
    nexwick::parse_nexus_file(path)
        .map(|(trees, label_map)| {
            let py_trees = trees.into_iter().map(PyCompactTree::new).collect();
            (py_trees, PyLeafLabelMap::new(label_map))
        })
        .map_err(|e| PyValueError::new_err(e.to_string()))
}
