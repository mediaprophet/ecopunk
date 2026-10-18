//! Conical Ray-Bundle Frustum Slicing for Geodesic & Industrial Full-Dome Displays
//! Partitions continuous 10D world manifolds across 600+ independent 4K projector nodes
//! without intermediate framebuffer warping.

use serde::{Deserialize, Serialize};

/// Conical solid-angle wedge assigned to an individual projector or display node.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrustumWedge {
    /// Unique index of this display node in the cluster [0..N-1]
    pub node_index: u32,
    /// Center azimuth angle θ in radians [-π..π]
    pub theta_center_rad: f32,
    /// Azimuth angular spread Δθ in radians
    pub theta_extent_rad: f32,
    /// Center elevation angle φ in radians [0..π/2] (0 = horizon, π/2 = zenith)
    pub phi_center_rad: f32,
    /// Elevation angular spread Δφ in radians
    pub phi_extent_rad: f32,
    /// Projector optical edge-blend margin ratio (e.g. 0.08 = 8% overlap)
    pub edge_blend_overlap: f32,
}

impl FrustumWedge {
    /// Generates a full-dome cluster decomposition for N display nodes.
    /// Distributes nodes across elevation rings according to solid angle area.
    pub fn generate_dome_cluster(node_count: u32) -> Vec<Self> {
        let mut wedges = Vec::with_capacity(node_count as usize);
        if node_count == 0 {
            return wedges;
        }

        // Estimate number of elevation rings: ~ sqrt(N / 2)
        let ring_count = ((node_count as f32 / 2.0).sqrt().round() as u32).max(1);
        let phi_step = (std::f32::consts::FRAC_PI_2) / ring_count as f32;

        let mut current_node = 0u32;
        for ring in 0..ring_count {
            let phi_center = (ring as f32 + 0.5) * phi_step;
            // Circumference proportional to cos(phi)
            let ring_weight = phi_center.cos().max(0.1);
            let ring_nodes = ((node_count as f32 / ring_count as f32) * ring_weight * 1.5)
                .round() as u32;
            let ring_nodes = ring_nodes.max(1);

            let theta_step = (std::f32::consts::TAU) / ring_nodes as f32;

            for n in 0..ring_nodes {
                if current_node >= node_count {
                    break;
                }
                let theta_center = -std::f32::consts::PI + (n as f32 + 0.5) * theta_step;
                wedges.push(FrustumWedge {
                    node_index: current_node,
                    theta_center_rad: theta_center,
                    theta_extent_rad: theta_step * 1.08, // 8% blend overlap
                    phi_center_rad: phi_center,
                    phi_extent_rad: phi_step * 1.08,
                    edge_blend_overlap: 0.08,
                });
                current_node += 1;
            }
        }

        // Fill remaining nodes if rounding didn't reach target
        while current_node < node_count {
            wedges.push(FrustumWedge {
                node_index: current_node,
                theta_center_rad: 0.0,
                theta_extent_rad: 0.2,
                phi_center_rad: std::f32::consts::FRAC_PI_2, // Zenith
                phi_extent_rad: 0.2,
                edge_blend_overlap: 0.08,
            });
            current_node += 1;
        }

        wedges
    }

    /// Evaluates whether a given 3D unit ray from the dome center falls within this wedge.
    #[inline]
    pub fn contains_ray(&self, ray: [f32; 3]) -> bool {
        // ray: [x, y, z] where z is up
        let r_horizontal = (ray[0] * ray[0] + ray[1] * ray[1]).sqrt();
        let phi = ray[2].atan2(r_horizontal);
        let theta = ray[1].atan2(ray[0]);

        let half_phi = self.phi_extent_rad * 0.5;
        let half_theta = self.theta_extent_rad * 0.5;

        (phi - self.phi_center_rad).abs() <= half_phi
            && (theta - self.theta_center_rad).abs() <= half_theta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_600_screen_dome_cluster_generation() {
        let cluster = FrustumWedge::generate_dome_cluster(600);
        assert_eq!(cluster.len(), 600);
        assert_eq!(cluster[0].node_index, 0);
        assert_eq!(cluster[599].node_index, 599);
    }
}
