
/// Error for the checked triangle-ordered connectivity traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderedConnectivityError {
    SizeOverflow,
    HalfEdgeLengthMismatch { required: usize, actual: usize },
    LabelBufferTooSmall { required: usize },
    QueueBufferTooSmall { required: usize },
    WorkLimitExceeded { required: usize },
    InvalidLayout { index: u32 },
}

/// Conservative work units for checked triangle-ordered validation and BFS.
/// This is a deterministic accounting bound, not a measurement of CPU time.
pub fn triangle_ordered_component_work(face_count: u32) -> Result<usize, OrderedConnectivityError> {
    let faces=face_count as usize;
    if face_count > u32::MAX/3 { return Err(OrderedConnectivityError::SizeOverflow); }
    let edges=faces.checked_mul(3).ok_or(OrderedConnectivityError::SizeOverflow)?;
    edges.checked_mul(16).and_then(|x|x.checked_add(faces.checked_mul(8)?)).and_then(|x|x.checked_add(64)).ok_or(OrderedConnectivityError::SizeOverflow)
}

/// Label components in the checked contiguous triangle layout emitted by
/// `topology::build_triangle_half_edges`. Unlike the general `label_components`
/// API, which accepts arbitrary half-edge ordering, this operation visits only
/// each face's three half-edges during BFS. Identical input yields identical
/// labels. `labels` and `queue` are caller-owned; no heap allocation occurs.
///
/// Before writing either buffer, the function checks buffer lengths, work cap,
/// exact `3 * face_count` length, face/next layout and all paired reverse twins.
/// Invalid inputs refuse without modifying buffers. The work bound covers
/// validation and traversal, and is independent of hash-probe costs of the
/// separate half-edge constructor. This validates topological layout only; it
/// does not establish geometric manifoldness or absence of self-intersections.
pub fn label_components_triangle_ordered(
    face_count:u32, half_edges:&[HalfEdge], max_work:usize,
    labels:&mut [u32], queue:&mut [u32],
) -> Result<u32,OrderedConnectivityError> {
    let required_work=triangle_ordered_component_work(face_count)?;
    if max_work<required_work { return Err(OrderedConnectivityError::WorkLimitExceeded{required:required_work}); }
    let faces=face_count as usize;
    let required=faces.checked_mul(3).ok_or(OrderedConnectivityError::SizeOverflow)?;
    if half_edges.len()!=required { return Err(OrderedConnectivityError::HalfEdgeLengthMismatch{required,actual:half_edges.len()}); }
    if labels.len()<faces { return Err(OrderedConnectivityError::LabelBufferTooSmall{required:faces}); }
    if queue.len()<faces { return Err(OrderedConnectivityError::QueueBufferTooSmall{required:faces}); }
    for (i,edge) in half_edges.iter().enumerate() {
        let face=i/3;let base=face*3;
        let next=(base+(i%3+1)%3) as u32;
        if edge.face!=face as u32 || edge.next!=next { return Err(OrderedConnectivityError::InvalidLayout{index:i as u32}); }
        if edge.twin!=INVALID_INDEX {
            let t=edge.twin as usize;
            if t>=required || half_edges[t].twin!=i as u32 || half_edges[t].face==edge.face {
                return Err(OrderedConnectivityError::InvalidLayout{index:i as u32});
            }
            let tn=half_edges[t].next as usize;
            if tn>=required || half_edges[t].origin!=half_edges[next as usize].origin || half_edges[tn].origin!=edge.origin {
                return Err(OrderedConnectivityError::InvalidLayout{index:i as u32});
            }
        }
    }
    labels[..faces].fill(INVALID_INDEX);
    let mut components=0u32;
    for seed in 0..faces {
        if labels[seed]!=INVALID_INDEX {continue;}
        let mut head=0usize;let mut tail=1usize;
        queue[0]=seed as u32;labels[seed]=components;
        while head<tail {
            let face=queue[head] as usize;head+=1;
            let base=face*3;
            for edge in &half_edges[base..base+3] {
                if edge.twin==INVALID_INDEX {continue;}
                let neighbor=half_edges[edge.twin as usize].face as usize;
                if labels[neighbor]==INVALID_INDEX {
                    labels[neighbor]=components;queue[tail]=neighbor as u32;tail+=1;
                }
            }
        }
        components+=1;
    }
    Ok(components)
}
