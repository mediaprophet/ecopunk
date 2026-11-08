# Made world — human-made things, assemblies and systems

**Status:** specification draft v0.1 · **Updated:** 2026-10-24.
Timothy's “world of man” distinction is a domain requirement alongside the
natural “world of god.” These labels describe the intended scope; the technical
names below do not assert a theological account or turn living beings into things.

Companions: [domain model](domain-model.md), [architecture](architecture.md),
[HMC package](tech/assets/asset-package-spec.md),
[interactive asset requirements](tech/assets/digital-twin-asset-requirements.md).

## 1. Domain boundary

The made world contains designs and human-made artefacts: vehicles, machines,
tools, structures, buildings, infrastructure, manufactured materials/products
and assemblies. It requires more structure than a generic `Model3DAsset` or a
mesh with a category label. The proposed `made:` family describes the thing's
design, manufacture, components, operation, condition and history.

Natural entities retain `bio:` / `env:` identifiers. People, communities, ownership,
custody and authority retain `soc:` relationships. A planted tree remains an
organism; its planter, irrigation hardware and greenhouse are made-world
artefacts. A natural timber source, harvested batch, manufactured beam, installed
beam and its depiction are linked by provenance/acts, not collapsed into one ID.
Making a representation of an animal does not make that animal manufactured.

`made:` names in this draft are conceptual vocabulary proposals under the
project's canonical namespace policy, not a published ontology or new codec.

## 2. Entity structure

| Class / role | Meaning and distinction |
|---|---|
| Artefact family/type | Broad kind such as vehicle, pump or workshop; carries applicability rules. |
| Design definition / revision | Intended design, plans, parameters, tolerances, specified parts and compatibility; not an observed physical object. |
| Manufactured instance | A particular built subject with identifiers, build provenance and actual as-built configuration; serial identification only where evidenced. |
| Assembly / subassembly | Persistent whole made of identifiable parts; not merely a combined drawing mesh. |
| Component definition / instance | Specified part versus particular fitted part; can be removed, replaced, reused or retired with lineage. |
| Material specification / batch | Design material and quantities versus sourced batch with physical properties and provenance. |
| Port / connector / interface | Typed electrical, mechanical, fluid, thermal, data or other connection point with frame and operating constraints. |
| Functional subsystem | Connected parts performing a function, e.g. drive, water filtration or electrical storage; may cross assembly boundaries. |
| Installation / site configuration | Placement, anchors, service connections and configuration of particular instances in a place/time. |
| Operational model / control policy | Declared solver/rules, controls, preconditions and effects; separates intended function from qualified performance. |
| Observation / inspection | Evidence of condition, measurement or operating outcome; not interchangeable with a design rating. |
| Event / work record | Manufacture, installation, operation, fault, modification, maintenance, repair, transfer, dismantling or recycling. |
| Representation | CAD, realistic or illustrated model, assembly diagram, section, icon or other depiction of the above. |

Product/trade listing is an additional act-derived role. A manufactured artefact
need not be offered for sale. A legal ownership claim, custody, use permission,
maintenance responsibility and physical possession are distinct relationships.

## 3. Required relations and structure

A made-world embodiment must connect design → as-built instance → current
configuration → state/history → representations. Assembly membership and
functional connectivity are different graphs: a battery can be part of a
vehicle while participating in the site's charging system when connected.

| Relation family | Structural rule |
|---|---|
| Instance → design revision | An instance declares its design basis or records it as unknown; later modifications do not rewrite the original design. |
| Assembly → fitted component | At a configuration revision, each installed part has one structural parent or is explicitly an unattached root. Containment is acyclic; multiple joint/connection relationships are separate. |
| Component → subsystem | A part may serve several functional systems without being physically duplicated or assigned conflicting instance identifiers. |
| Port → connection → port | Connections bind actual endpoint identifiers and their compatibility/state; a fluid loop or electrical circuit is not an illegal containment cycle. |
| Instance → installation → place/time | Placement, anchoring and services are contextual/versioned; moving a vehicle does not create a new vehicle. |
| Work event → inputs/outputs/affected configuration | Removal, replacement, repair and material transformation retain participants, quantities, evidence and prior/subsequent revisions. |
| Representation → referent/part | Depiction membership is not physical assembly membership; a schematic can show a system spanning several artefacts. |
| Space → occupant/activity | Occupancy is a contextual relation, never manufactured-part containment or an ownership claim. |

These relations are semantic contracts, not proposed binary fields. Their
vocabulary bindings must follow the existing ontology review rules. Ordinary
graph cycles, ecological feedback, service loops and kinematic loops remain
permitted under their declared query/solver semantics. Cycle rejection in
package admission concerns invalid containment/dependency traversal, not a ban
on cyclic knowledge or functioning systems.

| ID | Requirement | Acceptance case |
|---|---|---|
| REQ-MADE-001 | Design/type, manufactured instance, component, material batch, depiction and release MUST have separate stable identifiers. | Repainting a rover changes representation/configuration evidence, not its design ID or manufacturing identifiers. |
| REQ-MADE-002 | Complete artefacts/families MUST package their graph, sources, assemblies, representations and required operational resources in HMC. | Native inspection can discover a whole vehicle/building and drill into a named part without a game-specific registry. |
| REQ-MADE-003 | Assemblies MUST declare stable membership, quantity, parent/attachment frame, units, placement and permitted joint/constraint relationships. | Body, wheels and doors assemble once at correct scale; invalid cycles/attachments are refused. |
| REQ-MADE-004 | Specified BOM and actual fitted configuration MUST remain distinct and versioned. | A replacement battery has its own identifiers; the removal/install event preserves the earlier configuration and material lineage. |
| REQ-MADE-005 | Materials and physical properties MUST be typed, sourced and condition-dependent where applicable. | Visual metal/paint settings do not invent mass, structural strength, conductivity or flammability. |
| REQ-MADE-006 | Functional ports/connections MUST declare medium, direction, units, compatibility, capacities and required permissions/conditions. | An incompatible electrical supply, fluid medium or attachment is refused before operation; actual flow remains distinct from a rated capacity. |
| REQ-MADE-007 | Operations MUST declare inputs, outputs, energy/material accounting, controls, preconditions and failure modes where modeled. | Pump motion cannot claim water delivery without an admitted supply, destination and flow result. Unknown performance remains unknown. |
| REQ-MADE-008 | Observed/as-built state, design rating, simulated prediction and illustrative animation MUST be distinguishable. | A concept solar array is not reported as installed generation; a simulation output is not a site measurement. |
| REQ-MADE-009 | Lifecycle MUST include relevant manufacture, install, operate, degrade/fail, inspect, maintain, repair, modify, dismantle and reuse/disposal events. | Damage and repair alter identified components/resources and retain the causal record. Inapplicable events have explicit reasons. |
| REQ-MADE-010 | Physical scale, articulation, collision and spatial service/clearance envelopes MUST bind to the same parts and qualified model. | Wheel/door motion, operating access and installation placement remain coherent through representation changes. |
| REQ-MADE-011 | Authority, ownership/custody/use/maintenance claims and licence/provenance MUST be explicit and separate from physical composition. | Being in a building does not make a person its component or property. An artefact model licence does not grant control of its real referent. |
| REQ-MADE-012 | Style/LOD/diagram changes MUST preserve design/instance/part state and admit only compatible representations. | Illustrated rover and CAD section refer to the same assembly; a simplified schematic declares omitted geometry and operation limits. |
| REQ-MADE-013 | Native Vibe/engine paths MUST own query, assembly admission, operations, accounting and state transitions. | A control UI submits an action; it cannot independently simulate pumps or copy graph facts into a JS state engine. |
| REQ-MADE-014 | A complete made-world model package MUST satisfy film, open-world game and cartoon delivery plus its full applicable operation/movement/behaviour inventory. | Per-deliverable and expanded-case claims follow the [completion contract](tech/assets/model-completion-claims.md); a prop/one-animation preview cannot claim a complete operational assembly. |

Interiors, doors, service spaces and access affordances belong in a structure's
assembly/operation definitions when claimed. Occupancy and people's activities
are contextual relations with their own privacy and authority, not bill-of-
materials entries. Demolition is not automatically modeled by showing fragments;
the selected physical model and its limits must be stated.

An operation starts as an intent against a named instance/configuration and
state revision. Native capability/authority, input availability, unit/connection
compatibility and physical preconditions are checked before admitted effects.
The owning engine applies those effects transactionally, accounts for declared
inputs/outputs and records the event and resulting state in the qualified
journal. Presentation follows that result. A stale intent, denied action or
failed resource reservation leaves no hidden water/energy/parts consumption.

## 4. Worked example: Community Grounds

The user-supplied reference `C:/Users/tim_h/Downloads/communitygrounds.jpg`
shows an illustrated place containing trees, people, vehicles, a hub building,
solar/battery equipment, hydroponic beds and water equipment. Its labels are
concept annotations, not measured capacities, proven operation or authorization
to perform any depicted process. No new reuse licence is inferred from the image.

| Scene subject | Required domain composition |
|---|---|
| Ecological survey vehicle / mobile workshop | Artefact design + built/synthetic instance; chassis/body, drivetrain, wheels, cabin, storage and optional services; operation and maintenance state. |
| Civic hub / workshop | Building assembly; structure/envelope/interior, openings, spaces, equipment and service ports; installation and activity context. |
| Solar microgrid / battery storage | Electrical subsystem of panel, inverter, battery and protection/control artefacts, with connection graph and declared power/energy states. |
| Water treatment / hydroponic equipment | Tanks, pipes, valves, pumps, treatment/bed modules and fluid ports; material/energy flow and maintenance. Plants remain natural organisms connected through cultivation/flow relations. |
| Nursery and gardens | Living instances supplied by species HMCs, arranged in place, with separate manufactured containers/tools and evidenced husbandry actions. |
| Trees / land / water | Natural organisms and environmental entities; shared spatial context does not make them artefact components. |
| People / cooperative activity | Sociosphere agents, roles and acts; an avatar depiction does not transfer authority or identifiers. |

The place is a **scene/site composition**, not one undifferentiated object mesh
and not one species HMC. It references species embodiments and artefact HMCs,
binds individual instances to place/time, and describes connections and governed
activities. The scene package includes the dependencies required for its
declared offline scenario; it may share admitted resources by identifiers.

## 5. Representation and operational depth

An illustrated scene may use simplified exteriors while its HMC retains richer
design, graph and operational information. Other consumers can request CAD,
interior, assembly-exploded, flow schematic or realistic representations of the
same thing. This is the purpose of separating embodiment from depiction.

A nonfunctional prop remains useful when explicitly declared illustrative.
Claiming an operational pump, vehicle or structure requires qualified models
and effects. This specification does not establish engineering certification,
manufacturing readiness or safety from an image or a simulated result.

## 6. Specification fixtures and open decisions

Define a complete vehicle HMC and a workshop-with-services HMC. Specify native
inspection, assembly/part selection, style switching, configuration change,
one operation, one failure/repair sequence and journal replay. Add a shared
site connection fixture for electricity or water. Review negative cases for
wrong units, incompatible connectors, missing parts, stale configurations,
insufficient inputs and denied actions before allocating implementation.

Open: exact vocabulary bindings and reusable upstream schemas; design-vs-as-
built discrepancy model; BOM/connection serialization; port/solver compatibility;
identifiers across replacement/reconstruction; package-completeness queries for
artefact families. These are requirements for stack development, not permission
to implement a private Ecopunk simulation or invent codec fields.
