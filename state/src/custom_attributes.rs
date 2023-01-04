use std::fmt::Debug;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::Mutex;

use dioxus_core::AnyValue;
use dioxus_native_core::node::FromAnyValue;
use freya_common::NodeReferenceLayout;
use tokio::sync::mpsc::UnboundedSender;

/// Node Reference
#[derive(Clone)]
pub struct NodeReference(pub UnboundedSender<NodeReferenceLayout>);

impl PartialEq for NodeReference {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Display for NodeReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeReference").finish_non_exhaustive()
    }
}

/// Cursor reference
#[derive(Clone, Debug)]
pub struct CursorReference {
    pub positions: Arc<Mutex<Option<(f32, f32)>>>,
    pub agent: UnboundedSender<(usize, usize)>,
    pub id: Arc<Mutex<Option<usize>>>,
}

impl PartialEq for CursorReference {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Display for CursorReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CursorReference").finish_non_exhaustive()
    }
}

/// Group all the custom attribute types
#[derive(Clone, PartialEq)]
pub enum CustomAttributeValues {
    Reference(NodeReference),
    CursorReference(CursorReference),
}

impl Debug for CustomAttributeValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reference(_) => f.debug_tuple("Reference").finish(),
            Self::CursorReference(_) => f.debug_tuple("CursorReference").finish(),
        }
    }
}

impl FromAnyValue for CustomAttributeValues {
    fn from_any_value(b: &dyn AnyValue) -> Self {
        b.as_any()
            .downcast_ref::<CustomAttributeValues>()
            .unwrap()
            .clone()
    }
}
