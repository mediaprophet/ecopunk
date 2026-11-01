//! Derived inventory over the existing validated botanical source, never a graph owner.
use super::super::ValidatedSource;
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Disposition {
    /// The legacy projection starts separate chains at the root even at degree two.
    RootConnector,
    RootJunction,
    Junction,
    RetainedTerminal,
    RetainedDegreeTwo,
}

#[derive(Clone, Debug, Serialize)]
pub struct NodeDisposition {
    pub organ: usize,
    pub degree: usize,
    pub disposition: Disposition,
}

#[derive(Clone, Debug, Serialize)]
pub struct ChainPlan {
    /// Original authored indices, in the same order as the legacy sweep.
    pub organs: Vec<usize>,
    pub start_joint: Option<usize>,
    pub end_joint: Option<usize>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Incident {
    /// Child organ index identifies the existing directed source edge.
    pub edge_child: usize,
    pub chain: usize,
    pub at_start: bool,
    /// Distance fraction from the junction along its incident source edge.
    pub cut_fraction: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct JunctionPlan {
    pub organ: usize,
    pub incidents: Vec<Incident>,
}

#[derive(Clone, Debug, Serialize)]
pub struct EdgeSpan {
    pub edge_child: usize,
    /// Fractions in parent-to-child source direction. The open middle is retained.
    pub retained_start: f64,
    pub retained_end: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct ForkPlan {
    pub nodes: Vec<NodeDisposition>,
    pub chains: Vec<ChainPlan>,
    pub junctions: Vec<JunctionPlan>,
    pub edges: Vec<EdgeSpan>,
    pub structural_work: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct Cluster {
    /// Indices into ForkPlan.junctions, sorted in source order.
    pub junctions: Vec<usize>,
    pub interior_chains: Vec<usize>,
    /// (junction index, incident index), in source order.
    pub ports: Vec<[usize; 2]>,
}

pub(super) fn clusters(
    source: &ValidatedSource,
    plan: &ForkPlan,
    authored: &[Vec<String>],
    work_budget: usize,
    cancelled: &dyn Fn() -> bool,
) -> Result<(Vec<Cluster>, usize), String> {
    if authored.len() > plan.junctions.len() {
        return Err("Too many authored coupled-junction groups".into());
    }
    let mut work = 0;
    let mut assigned = vec![false; plan.junctions.len()];
    let mut groups = Vec::<Vec<usize>>::new();
    for keys in authored {
        if keys.is_empty() || keys.len() > plan.junctions.len() {
            return Err("Empty or oversized coupled-junction group".into());
        }
        let mut group = Vec::with_capacity(keys.len());
        for key in keys {
            if cancelled() {
                return Err("All-fork cluster planning cancelled".into());
            }
            charge(&mut work, plan.junctions.len() + 16, work_budget)?;
            let organ = *source
                .lookup
                .get(key)
                .ok_or("Unknown authored coupled junction")?;
            let joint = plan
                .junctions
                .iter()
                .position(|j| j.organ == organ)
                .ok_or("Coupled member is not a required junction")?;
            if assigned[joint] {
                return Err("A required junction occurs in multiple coupled memberships".into());
            }
            assigned[joint] = true;
            group.push(joint);
        }
        group.sort_unstable();
        groups.push(group);
    }
    for (joint, &is_assigned) in assigned.iter().enumerate() {
        if !is_assigned {
            groups.push(vec![joint]);
        }
    }
    groups.sort_by_key(|g| plan.junctions[g[0]].organ);
    let mut group_for_joint = vec![usize::MAX; plan.junctions.len()];
    for (group, members) in groups.iter().enumerate() {
        for &joint in members {
            group_for_joint[joint] = group;
        }
    }
    let mut result = Vec::with_capacity(groups.len());
    for (group, members) in groups.into_iter().enumerate() {
        if cancelled() {
            return Err("All-fork cluster planning cancelled".into());
        }
        let mut interior_chains = Vec::new();
        let mut reached = std::collections::BTreeSet::new();
        reached.insert(members[0]);
        // Connectivity is source-chain incidence, never spatial proximity. The
        // bounded fixed point also proves no unlisted fork lies inside a group.
        loop {
            let old = reached.len();
            for (index, chain) in plan.chains.iter().enumerate() {
                charge(
                    &mut work,
                    4 + 2 * members.len() + plan.chains.len(),
                    work_budget,
                )?;
                if let (Some(a), Some(b)) = (chain.start_joint, chain.end_joint) {
                    if group_for_joint[a] == group && group_for_joint[b] == group {
                        if reached.contains(&a) {
                            reached.insert(b);
                        }
                        if reached.contains(&b) {
                            reached.insert(a);
                        }
                        if !interior_chains.contains(&index) {
                            interior_chains.push(index);
                        }
                    }
                }
            }
            if reached.len() == old {
                break;
            }
        }
        if reached.len() != members.len() {
            return Err("Coupled junctions are not connected by adopted source chains".into());
        }
        let mut ports = Vec::new();
        for &joint in &members {
            for (incident, entry) in plan.junctions[joint].incidents.iter().enumerate() {
                charge(&mut work, 1 + interior_chains.len(), work_budget)?;
                if !interior_chains.contains(&entry.chain) {
                    ports.push([joint, incident]);
                }
            }
        }
        if ports.is_empty() {
            return Err("Coupled region has no retained external boundary".into());
        }
        result.push(Cluster {
            junctions: members,
            interior_chains,
            ports,
        });
    }
    Ok((result, work))
}

fn charge(work: &mut usize, amount: usize, budget: usize) -> Result<(), String> {
    *work = work
        .checked_add(amount)
        .ok_or("All-fork planning work overflow")?;
    if *work > budget {
        return Err("All-fork planning exceeds caller work".into());
    }
    Ok(())
}

pub(super) fn build(
    source: &ValidatedSource,
    cut_fraction: f64,
    work_budget: usize,
    cancelled: &dyn Fn() -> bool,
) -> Result<ForkPlan, String> {
    if !cut_fraction.is_finite() || cut_fraction <= 0.0 || cut_fraction >= 0.5 {
        return Err("All-fork cut fraction must be finite, positive and below one half".into());
    }
    let n = source.parents.len();
    if !(2..=256).contains(&n) || source.children.len() != n {
        return Err("All-fork plan requires the bounded validated botanical source".into());
    }
    let mut work = 0;
    charge(
        &mut work,
        n.checked_mul(16).ok_or("All-fork planning work overflow")?,
        work_budget,
    )?;
    let mut nodes = Vec::with_capacity(n);
    let mut joint_for_node = vec![None; n];
    let mut junctions = Vec::new();
    for organ in 0..n {
        if cancelled() {
            return Err("All-fork planning cancelled".into());
        }
        charge(&mut work, 1, work_budget)?;
        let degree = source.children[organ].len() + usize::from(source.parents[organ].is_some());
        let disposition = match (organ, degree) {
            (0, 2) => Disposition::RootConnector,
            (0, 3..) => Disposition::RootJunction,
            (_, 3..) => Disposition::Junction,
            (_, 1) => Disposition::RetainedTerminal,
            (_, 2) => Disposition::RetainedDegreeTwo,
            _ => return Err("Unconnected source organ has no all-fork disposition".into()),
        };
        if matches!(
            disposition,
            Disposition::RootConnector | Disposition::RootJunction | Disposition::Junction
        ) {
            joint_for_node[organ] = Some(junctions.len());
            junctions.push(JunctionPlan {
                organ,
                incidents: Vec::with_capacity(degree),
            });
        }
        nodes.push(NodeDisposition {
            organ,
            degree,
            disposition,
        });
    }
    let mut edge_chain = vec![None; n];
    let mut chains = Vec::new();
    // Exact source-order/maximal-chain partition used by the existing sweep.
    for child in 1..n {
        if cancelled() {
            return Err("All-fork planning cancelled".into());
        }
        let parent = source.parents[child].ok_or("Missing validated source parent")?;
        if parent != 0 && source.children[parent].len() == 1 {
            continue;
        }
        let mut organs = vec![parent, child];
        while source.children[*organs.last().ok_or("Empty source chain")?].len() == 1 {
            let last = *organs.last().ok_or("Empty source chain")?;
            organs.push(source.children[last][0]);
        }
        let chain_index = chains.len();
        for &edge in &organs[1..] {
            charge(&mut work, 1, work_budget)?;
            if edge_chain[edge].replace(chain_index).is_some() {
                return Err("A source edge occurs in multiple maximal chains".into());
            }
        }
        let start = organs[0];
        let end = *organs.last().ok_or("Empty source chain")?;
        let start_joint = joint_for_node[start];
        let end_joint = joint_for_node[end];
        if let Some(joint) = start_joint {
            junctions[joint].incidents.push(Incident {
                edge_child: organs[1],
                chain: chain_index,
                at_start: true,
                cut_fraction,
            });
        }
        if let Some(joint) = end_joint {
            junctions[joint].incidents.push(Incident {
                edge_child: end,
                chain: chain_index,
                at_start: false,
                cut_fraction,
            });
        }
        chains.push(ChainPlan {
            organs,
            start_joint,
            end_joint,
        });
    }
    let mut edges = Vec::with_capacity(n - 1);
    for child in 1..n {
        charge(&mut work, 1, work_budget)?;
        let parent = source.parents[child].ok_or("Missing source parent")?;
        if edge_chain[child].is_none() {
            return Err("A source edge was skipped by all-fork chain partition".into());
        }
        let retained_start = if joint_for_node[parent].is_some() {
            cut_fraction
        } else {
            0.0
        };
        let retained_end = if joint_for_node[child].is_some() {
            1.0 - cut_fraction
        } else {
            1.0
        };
        if retained_start >= retained_end {
            return Err("Adjacent junction cuts overlap on a source edge".into());
        }
        edges.push(EdgeSpan {
            edge_child: child,
            retained_start,
            retained_end,
        });
    }
    for joint in &junctions {
        charge(&mut work, joint.incidents.len(), work_budget)?;
        if joint.incidents.len() != nodes[joint.organ].degree {
            return Err("Required all-fork junction has incomplete incident-chain coverage".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for incident in &joint.incidents {
            if !seen.insert(incident.edge_child) {
                return Err("Required all-fork junction has duplicate incident edge".into());
            }
        }
    }
    Ok(ForkPlan {
        nodes,
        chains,
        junctions,
        edges,
        structural_work: work,
    })
}
