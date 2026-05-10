pub mod model;
pub mod parsing;

use model::py_compact_tree::PyCompactTree;
use model::py_compact_vertex::PyCompactVertex;
use model::py_label_map::PyLeafLabelMap;
use parsing::nexus::parse_nexus_file;
use pyo3::prelude::*;

#[pymodule]
fn nexwick_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyCompactTree>()?;
    m.add_class::<PyCompactVertex>()?;
    m.add_class::<PyLeafLabelMap>()?;
    m.add_function(wrap_pyfunction!(parse_nexus_file, m)?)?;
    Ok(())
}
