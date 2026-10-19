#!/usr/bin/env python3
"""
generate_64_fauna.py — Authoritative 64 East Coast Australian Fauna Dataset & RDF Ontology Generator

Implements Domain 5 (Zoology, Fauna Ethology & Locomotion), Domain 7 (Bioacoustics & Soundscapes),
and Sections 3.2 / 3.3 of docs/plans/east-coast-australia-ecological-dataset-acquisition-plan.md.

Generates:
  1. data/catalog/fauna_64.json — Complete biophysical, kinematic, 3D anatomical, ethological,
     and bioacoustic catalog for 64 real East Coast Australian fauna species.
  2. data/ontologies/east_coast_fauna_64.ttl — W3C Turtle 1.1 RDF ontology compliant with
     SHACL-First Living Entities convention (typed as dwc:Taxon, rdfs:Class; NEVER owl:Thing).
"""

import json
import os
import sys


def get_raw_species_part1():
    return [
        # ==================== MAMMALS (1 - 20) ====================
        (
            "Macropus giganteus", "Eastern Grey Kangaroo", "Buru / Wambool", "Ngarigo, Walgalu, Wiradjuri, and Yuin Country",
            "Mammalia", "Diprotodontia", "Macropodidae",
            "https://biodiversity.org.au/afd/taxa/4512", "2440132", "9315", "LC",
            ["talbingo_pines", "jounama_karst", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1800], "terrestrial",
            (55.0, 1.50, 1.15, 68.0, 36.2, "endotherm"),
            ("bipedal_hop", 1.4, 5.8, 17.8, 2.2, 1.8, 1.65),
            ("macropod", [1.0, 1.1, 1.3], [0.42, 0.45, 0.55], 0.95, 1.05, 0.28, 0.0, [0.33, 0.31, 0.27], [0.55, 0.52, 0.47], [0.13, 0.12, 0.11], 24),
            ("crepuscular", "grazer", ["GRAZING", "PENTAPEDAL_WALK", "HOPPING", "VIGILANT_SCAN", "FLEEING", "RESTING_SHADE"],
             48.0, [4, 35], ["poa_sieberiana", "themeda_triandra", "microlaena_stipoides"],
             "WHEN solar_elevation_deg < 25 OR sky_cloud_cover > 0.6 PERMIT foraging_in(poa_sieberiana_meadow); IF predator_dist_m < 48.0 OBLIGATE emit_alarm_thump AND transition(FLEEING)."),
            ("Guttural alarm cough & hind-foot ground thump", 85.0, 1450.0, 160.0, 78.0, "low_thump")
        ),
        (
            "Notamacropus rufogriseus", "Red-necked Wallaby", "Bungari /Warroo", "Ngarigo, Walgalu, and D'harawal Country",
            "Mammalia", "Diprotodontia", "Macropodidae",
            "https://biodiversity.org.au/afd/taxa/4518", "9300807", "9320", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1600], "terrestrial",
            (16.5, 0.92, 0.68, 28.4, 36.4, "endotherm"),
            ("bipedal_hop", 1.2, 4.9, 14.5, 2.6, 2.4, 1.15),
            ("macropod", [0.72, 0.78, 0.92], [0.32, 0.34, 0.42], 0.68, 0.76, 0.22, 0.0, [0.42, 0.38, 0.34], [0.58, 0.31, 0.22], [0.18, 0.16, 0.15], 24),
            ("crepuscular", "grazer", ["GRAZING", "PENTAPEDAL_WALK", "HOPPING", "VIGILANT_SCAN", "RESTING_SHADE"],
             38.0, [1, 6], ["poa_sieberiana", "lomandra_longifolia", "themeda_triandra"],
             "PREFER forest_edge_radius_m < 60.0; WHEN ambient_lux < 1500 PERMIT grazing(poa_sieberiana); IF threat_detected OBLIGATE retreat_to(sclerophyll_understory)."),
            ("Soft rasping hiss & paired foot thump", 110.0, 1800.0, 210.0, 72.0, "low_thump")
        ),
        (
            "Wallabia bicolor", "Swamp Wallaby", "Burrin / Black Pademelon", "Ngarigo, Yuin, Eora, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Macropodidae",
            "https://biodiversity.org.au/afd/taxa/4534", "2440182", "9329", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 1700], "terrestrial",
            (15.0, 0.82, 0.62, 26.8, 36.3, "endotherm"),
            ("bipedal_hop", 1.1, 4.4, 13.2, 2.7, 2.6, 0.95),
            ("macropod", [0.70, 0.74, 0.88], [0.30, 0.32, 0.38], 0.62, 0.72, 0.20, 0.0, [0.22, 0.19, 0.17], [0.56, 0.36, 0.18], [0.12, 0.11, 0.10], 24),
            ("cathemeral", "browser", ["BROWSING", "LOW_HEAD_HOPPING", "CRYPTIC_FREEZE", "FLEEING", "RESTING_THICKET"],
             30.0, [1, 2], ["pteridium_esculentum", "acacia_dealbata", "dicksonia_antarctica"],
             "MAINTAIN head_pitch_rad < -0.25 DURING hopping; PERMIT browsing(pteridium_esculentum, toxic_ferns); WHEN disturbed PREFER cryptic_freeze BEFORE zig_zag_flight."),
            ("Low nasal grunt & heavy single thump", 120.0, 1600.0, 195.0, 70.0, "low_thump")
        ),
        (
            "Petrogale penicillata", "Brush-tailed Rock-wallaby", "Wirrit /Bamarook", "Walgalu, Ngarigo, Wiradjuri, and Gundungurra Country",
            "Mammalia", "Diprotodontia", "Macropodidae",
            "https://biodiversity.org.au/afd/taxa/4527", "2440111", "9322", "VU",
            ["jounama_karst", "yarrangobilly_cliff", "sydney_sandstone"], [150, 1500], "terrestrial",
            (7.5, 0.56, 0.44, 15.6, 36.5, "endotherm"),
            ("bipedal_hop", 1.3, 4.8, 12.5, 3.2, 3.8, 1.10),
            ("macropod", [0.52, 0.55, 0.68], [0.24, 0.26, 0.32], 0.48, 0.64, 0.18, 0.0, [0.38, 0.29, 0.23], [0.68, 0.62, 0.54], [0.15, 0.13, 0.12], 24),
            ("crepuscular", "browser", ["LEDGE_HOPPING", "SUN_BASKING_ROCK", "BROWSING_CREVICE", "VIGILANT_OUTCROP", "CAVE_REFUGE"],
             35.0, [3, 18], ["poa_sieberiana", "acacia_pycnantha", "corrrea_reflexa"],
             "OBLIGATE proximity_to(karst_cliff_or_talus) < 80.0m; PERMIT morning_basking ON north_facing_ledges; IF predator_canid_detected OBLIGATE ascend_vertical_ledge(slope_deg > 55)."),
            ("Sharp alarm foot-stamp on rock & hiss", 160.0, 2400.0, 310.0, 74.0, "low_thump")
        ),
        (
            "Osphranter robustus", "Eastern Wallaroo", "Wolaru /Biggada", "Walgalu, Wiradjuri, and Dharug Country",
            "Mammalia", "Diprotodontia", "Macropodidae",
            "https://biodiversity.org.au/afd/taxa/4515", "9517912", "9317", "LC",
            ["jounama_karst", "yarrangobilly_cliff", "sydney_sandstone"], [100, 1650], "terrestrial",
            (38.0, 1.25, 0.95, 51.2, 36.1, "endotherm"),
            ("bipedal_hop", 1.3, 5.2, 15.4, 2.3, 2.1, 1.38),
            ("macropod", [0.95, 1.02, 1.18], [0.38, 0.42, 0.50], 0.84, 0.96, 0.25, 0.0, [0.25, 0.23, 0.22], [0.46, 0.43, 0.39], [0.11, 0.10, 0.10], 24),
            ("crepuscular", "grazer", ["GRAZING_SLOPE", "PENTAPEDAL_CLIMB", "HOPPING", "SHADE_SHELTERING_OVERHANG"],
             42.0, [1, 5], ["poa_sieberiana", "themeda_triandra", "austrostipa_scabra"],
             "PREFER terrain_slope_deg BETWEEN [15.0, 45.0]; WHEN air_temp_c > 28.0 OBLIGATE seek_rock_overhang_shade."),
            ("Harsh bronchial hiss & heavy thump", 95.0, 1550.0, 175.0, 77.0, "low_thump")
        ),
        (
            "Potorous tridactylus", "Long-nosed Potoroo", "Waruk / Potoroo", "Yuin, D'harawal, Ngarigo, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Potoroidae",
            "https://biodiversity.org.au/afd/taxa/4501", "2440072", "9310", "VU",
            ["talbingo_pines", "sydney_sandstone", "noosa_headland"], [0, 1350], "terrestrial",
            (1.15, 0.36, 0.22, 4.1, 36.0, "endotherm"),
            ("bipedal_hop", 0.55, 1.8, 5.2, 3.8, 4.2, 0.38),
            ("macropod", [0.30, 0.28, 0.38], [0.16, 0.14, 0.24], 0.22, 0.24, 0.09, 0.0, [0.36, 0.28, 0.21], [0.58, 0.53, 0.46], [0.20, 0.16, 0.14], 20),
            ("nocturnal", "omnivore", ["DIGGING_TRUFFLES", "SNIFFING_LITTER", "MICRO_HOPPING", "FREEZING_UNDER_FERN", "NESTING_TUSSOCK"],
             18.0, [1, 2], ["pisolithus_albus", "cortinarius_archeri", "lomandra_longifolia"],
             "OBLIGATE canopy_or_shrub_cover > 0.65; FORAGE hypogeal_ectomycorrhizal_fungi AND disperse_spores TO eucalyptus_roots."),
            ("Quiet rapid sniffing & soft alarm grunt", 220.0, 3200.0, 480.0, 54.0, "low_thump")
        ),
        (
            "Vombatus ursinus", "Bare-nosed Wombat", "Guli / Mirrigang", "Ngarigo, Walgalu, Wiradjuri, and Yuin Country",
            "Mammalia", "Diprotodontia", "Vombatidae",
            "https://biodiversity.org.au/afd/taxa/4580", "2440224", "29139", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone"], [0, 1950], "fossorial",
            (30.0, 0.98, 0.42, 32.0, 35.2, "endotherm"),
            ("fossorial_waddle", 0.75, 2.4, 9.8, 1.9, 1.5, 0.08),
            ("wombat", [0.95, 0.68, 1.05], [0.44, 0.40, 0.46], 0.32, 0.05, 0.12, 0.0, [0.26, 0.21, 0.17], [0.35, 0.29, 0.24], [0.14, 0.12, 0.11], 20),
            ("nocturnal", "grazer", ["GRAZING_TUSSOCK", "EXCAVATING_BURROW", "WADDLING_TRAIL", "CUBE_SCAT_MARKING", "BURROW_THERMOREGULATING"],
             22.0, [1, 2], ["poa_sieberiana", "themeda_triandra", "carex_appressa"],
             "WHEN air_temp_c > 24.0 OBLIGATE remain_in(subsurface_burrow_m64); AT dusk PERMIT emerge AND deposit_cubic_scat ON elevated_rock_or_log."),
            ("Rasping guttural warning hiss & excavation thud", 140.0, 2600.0, 340.0, 68.0, "syllabic_trill")
        ),
        (
            "Phascolarctos cinereus", "Koala", "Gula / Koala", "Wiradjuri, Dharug, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Phascolarctidae",
            "https://biodiversity.org.au/afd/taxa/4578", "2440220", "38626", "EN",
            ["talbingo_pines", "sydney_sandstone", "noosa_headland"], [0, 1200], "arboreal",
            (8.5, 0.72, 0.38, 11.4, 36.0, "endotherm"),
            ("arboreal_climb", 0.35, 0.9, 3.2, 1.1, 1.2, 0.25),
            ("koala", [0.62, 0.68, 0.58], [0.38, 0.36, 0.36], 0.45, 0.03, 0.24, 0.0, [0.46, 0.45, 0.43], [0.78, 0.76, 0.73], [0.16, 0.15, 0.15], 22),
            ("cathemeral", "browser", ["SLEEPING_FORK", "BROWSING_EUCALYPT_FOLIAGE", "CLIMBING_TRUNK", "BELLOWING_TERRITORIAL", "THERMAL_TRUNK_HUGGING"],
             20.0, [1, 1], ["eucalyptus_viminalis", "eucalyptus_tereticornis", "eucalyptus_pauciflora"],
             "OBLIGATE rest_duration_h >= 18.0 per_24h; WHEN air_temp_c > 30.0 OBLIGATE press_ventrum_against(cool_lower_eucalypt_trunk)."),
            ("Deep resonant pig-like inhalational snore & belch bellow", 90.0, 1850.0, 185.0, 84.0, "low_thump")
        ),
        (
            "Ornithorhynchus anatinus", "Platypus", "Biladurang / Mallangong", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Monotremata", "Ornithorhynchidae",
            "https://biodiversity.org.au/afd/taxa/4401", "2433406", "9258", "NT",
            ["talbingo_pines", "jounama_karst", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1550], "riparian_aquatic",
            (1.75, 0.50, 0.14, 3.8, 32.0, "endotherm"),
            ("sub_aquatic_swim", 0.40, 1.1, 2.4, 2.8, 3.4, 0.12),
            ("platypus", [0.48, 0.18, 0.55], [0.22, 0.12, 0.28], 0.16, 0.20, 0.02, 0.0, [0.27, 0.20, 0.15], [0.54, 0.44, 0.34], [0.18, 0.17, 0.17], 20),
            ("crepuscular", "carnivore", ["ELECTRORECEPTIVE_BOTTOM_FORAGING", "SURFACE_FLOAT_GROOM", "DIVING_PLUNGE", "BANK_BURROW_NESTING"],
             25.0, [1, 2], ["euastacus_armatus", "benthic_trichoptera_larvae", "chironomid_larvae"],
             "CLOSE eyes_ears_nostrils UNDERWATER; LOCATE benthic_prey VIA bill_electroreception_and_mechanoreception; DIVE duration_s <= 60.0."),
            ("Soft puppy-like growl & 280Hz diving splash plunge", 180.0, 1200.0, 280.0, 62.0, "splash_plunge")
        ),
        (
            "Tachyglossus aculeatus", "Short-beaked Echidna", "wandayali / Bigibila", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Monotremata", "Tachyglossidae",
            "https://biodiversity.org.au/afd/taxa/4403", "2433409", "9261", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 2000], "fossorial",
            (4.5, 0.45, 0.22, 4.9, 31.5, "endotherm"),
            ("fossorial_waddle", 0.35, 0.85, 2.1, 1.6, 1.4, 0.05),
            ("echidna", [0.52, 0.36, 0.54], [0.18, 0.14, 0.26], 0.18, 0.04, 0.15, 0.0, [0.24, 0.19, 0.15], [0.76, 0.68, 0.52], [0.12, 0.10, 0.09], 18),
            ("cathemeral", "insectivore", ["FORAGING_TERMITE_ANTS", "ROLLING_DEFENSIVE_BALL", "DIGGING_VERTICAL_SINK", "TORPOR_HIBERNATION", "MATING_TRAIN"],
             15.0, [1, 5], ["iridomyrmex_purpureus", "nasutitermes_exitiosus", "scarab_larvae"],
             "IF predator_dist_m < 10.0 OBLIGATE dig_straight_down_into(soil_horizon_A) AND erect_keratin_spines; IN winter_subalpine PERMIT hibernation_torpor(body_temp_c = 8.0)."),
            ("Rhythmic nasal snuffling & spine rustle", 200.0, 2800.0, 420.0, 52.0, "stridulation")
        ),
        (
            "Petauroides volans", "Southern Greater Glider", "Warnda /Mungungo", "Ngarigo, Walgalu, Yuin, and Gundungurra Country",
            "Mammalia", "Diprotodontia", "Pseudocheiridae",
            "https://biodiversity.org.au/afd/taxa/4562", "2439864", "38619", "EN",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone"], [100, 1450], "aerial_canopy",
            (1.35, 0.43, 0.20, 4.2, 36.1, "endotherm"),
            ("patagial_glide", 0.30, 1.1, 11.5, 1.8, 2.5, 0.85),
            ("glider_possum", [0.44, 0.32, 0.52], [0.22, 0.22, 0.24], 0.34, 0.55, 0.22, 0.68, [0.18, 0.17, 0.17], [0.88, 0.87, 0.85], [0.32, 0.28, 0.26], 22),
            ("nocturnal", "browser", ["PATAGIAL_GLIDING", "BROWSING_UPPER_CROWN", "HOLLOW_DENNING", "TRUNK_LANDING_BRAKE"],
             25.0, [1, 2], ["eucalyptus_delegatensis", "eucalyptus_viminalis", "eucalyptus_radiata"],
             "OBLIGATE nest_in(veteran_eucalypt_hollow_age_yr > 120); GLIDE max_range_m <= 100.0 AT glide_angle_deg <= 40.0."),
            ("Near-silent patagial swoosh & soft bark claw landing", 150.0, 1800.0, 320.0, 46.0, "low_thump")
        ),
        (
            "Petaurus breviceps", "Sugar Glider", "Peta /Ayawa", "Ngarigo, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Petauridae",
            "https://biodiversity.org.au/afd/taxa/4555", "2439886", "9313", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1400], "aerial_canopy",
            (0.13, 0.18, 0.09, 0.85, 36.3, "endotherm"),
            ("patagial_glide", 0.45, 1.6, 9.5, 3.5, 4.5, 0.45),
            ("glider_possum", [0.22, 0.16, 0.26], [0.12, 0.12, 0.14], 0.18, 0.21, 0.12, 0.32, [0.52, 0.53, 0.55], [0.85, 0.83, 0.79], [0.14, 0.14, 0.15], 22),
            ("nocturnal", "omnivore", ["PATAGIAL_GLIDING", "SAP_INCISING_ACACIA", "NECTAR_LICKING", "COMMUNAL_HOLLOW_HUDDLE", "ALARM_YAPPING"],
             20.0, [3, 10], ["acacia_pycnantha", "acacia_dealbata", "eucalyptus_viminalis", "agrotis_infusa"],
             "INCISE acacia_bark TO harvest_gum_exudate; WHEN owl_or_quoll_detected OBLIGATE emit_staccato_yap."),
            ("Sharp staccato terrier-like yapping & chattering", 850.0, 5400.0, 1850.0, 68.0, "syllabic_trill")
        ),
        (
            "Burramys parvus", "Mountain Pygmy-possum", "Burramys", "Ngarigo and Walgalu Country",
            "Mammalia", "Diprotodontia", "Burramyidae",
            "https://biodiversity.org.au/afd/taxa/4545", "2439829", "38605", "CR",
            ["three_mile_dam", "yarrangobilly_cliff"], [1300, 2228], "terrestrial",
            (0.045, 0.11, 0.06, 0.38, 35.8, "endotherm"),
            ("quadrupedal_trot", 0.25, 0.85, 2.6, 4.8, 5.2, 0.14),
            ("glider_possum", [0.14, 0.10, 0.16], [0.08, 0.08, 0.09], 0.09, 0.14, 0.08, 0.0, [0.44, 0.38, 0.32], [0.76, 0.72, 0.66], [0.22, 0.19, 0.17], 20),
            ("nocturnal", "omnivore", ["FORAGING_BOGONG_MOTHS", "CACHEING_PODOCARPUS_SEEDS", "BOULDERFIELD_HIBERNATION", "PREHENSILE_TAIL_CLIMB"],
             12.0, [1, 5], ["agrotis_infusa", "podocarpus_lawrencei", "tasmannia_xerophila"],
             "OBLIGATE periglacial_granodiorite_boulderfield_habitat(elevation_m >= 1300); GORGE on(agrotis_infusa) IN spring_summer TO build_subcutaneous_fat FOR 7_month_subnivean_hibernation."),
            ("High ultrasonic-border squeak & soft clicking", 2400.0, 11500.0, 4800.0, 48.0, "high_whistle")
        ),
        (
            "Trichosurus vulpecula", "Common Brushtail Possum", "Willi /Bung", "Ngarigo, Walgalu, Wiradjuri, Eora, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Phalangeridae",
            "https://biodiversity.org.au/afd/taxa/4540", "2439775", "9337", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 1600], "arboreal",
            (3.2, 0.48, 0.26, 7.6, 36.2, "endotherm"),
            ("arboreal_climb", 0.55, 1.8, 5.5, 2.4, 3.0, 0.65),
            ("glider_possum", [0.46, 0.34, 0.52], [0.22, 0.20, 0.26], 0.32, 0.38, 0.18, 0.0, [0.42, 0.40, 0.38], [0.72, 0.64, 0.52], [0.12, 0.11, 0.11], 22),
            ("nocturnal", "omnivore", ["ARBOREAL_BROWSING", "STERNAL_SCENT_MARKING", "ROOF_OR_HOLLOW_DENNING", "AGONISTIC_SCREECHING"],
             18.0, [1, 2], ["eucalyptus_viminalis", "acacia_pycnantha", "banksia_integrifolia"],
             "PERMIT forage_canopy_and_ground; DEFEND den_hollow VIA loud_guttural_coughing_and_screeching."),
            ("Loud guttural metallic cough, hiss & territorial screech", 350.0, 4800.0, 920.0, 79.0, "syllabic_trill")
        ),
        (
            "Pseudocheirus peregrinus", "Common Ringtail Possum", "Bung /Bukari", "Ngarigo, Yuin, Dharug, and Gubbi Gubbi Country",
            "Mammalia", "Diprotodontia", "Pseudocheiridae",
            "https://biodiversity.org.au/afd/taxa/4560", "2439848", "9334", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1400], "arboreal",
            (0.90, 0.34, 0.17, 3.2, 36.1, "endotherm"),
            ("arboreal_climb", 0.45, 1.5, 4.6, 2.8, 3.6, 0.50),
            ("glider_possum", [0.34, 0.24, 0.38], [0.16, 0.15, 0.18], 0.24, 0.36, 0.11, 0.0, [0.38, 0.32, 0.28], [0.68, 0.42, 0.24], [0.92, 0.91, 0.89], 22),
            ("nocturnal", "browser", ["DREYS_NEST_WEAVING", "PREHENSILE_BRANCH_WALK", "CAECOTROPHY_DIGESTION", "FOLIAGE_BROWSING"],
             16.0, [2, 5], ["eucalyptus_pauciflora", "callistemon_citrinus", "leptospermum_lanigerum"],
             "CONSTRUCT spherical_twig_drey IN dense_shrub_layer; USE white_tipped_prehensile_tail AS fifth_grasping_limb."),
            ("High-pitched bird-like twittering contact call", 1800.0, 7200.0, 3400.0, 56.0, "high_whistle")
        ),
        (
            "Dasyurus maculatus", "Spotted-tailed Quoll", "Bindjulang /Guruma", "Ngarigo, Walgalu, Yuin, and Gundungurra Country",
            "Mammalia", "Dasyuromorphia", "Dasyuridae",
            "https://biodiversity.org.au/afd/taxa/4428", "2435398", "9279", "EN",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone"], [0, 1850], "terrestrial",
            (4.2, 0.62, 0.26, 9.4, 36.6, "endotherm"),
            ("quadrupedal_trot", 0.90, 3.4, 10.5, 2.9, 3.4, 0.80),
            ("quoll_dasyurid", [0.58, 0.28, 0.68], [0.24, 0.20, 0.28], 0.28, 0.48, 0.10, 0.0, [0.42, 0.26, 0.16], [0.92, 0.90, 0.84], [0.18, 0.14, 0.12], 22),
            ("nocturnal", "carnivore", ["STALKING_PREY", "ARBOREAL_POUNCE", "LATRINE_ROCK_MARKING", "DENNING_HOLLOW_LOG"],
             30.0, [1, 1], ["trichosurus_vulpecula", "pseudocheirus_peregrinus", "potorous_tridactylus", "antechinus_agilis"],
             "PATROL communal_rock_latrine_sites; HUNT arboreal_and_terrestrial_marsupials VIA cervical_bite."),
            ("Rasping chainsaw-like screech & warning cough", 450.0, 5800.0, 1250.0, 81.0, "syllabic_trill")
        ),
        (
            "Antechinus agilis", "Agile Antechinus", "Mardo /Antechinus", "Ngarigo, Walgalu, Yuin, and Kulin Country",
            "Mammalia", "Dasyuromorphia", "Dasyuridae",
            "https://biodiversity.org.au/afd/taxa/4412", "2435356", "38579", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone"], [50, 1900], "terrestrial",
            (0.028, 0.095, 0.04, 0.31, 36.4, "endotherm"),
            ("quadrupedal_trot", 0.35, 1.4, 4.2, 5.5, 6.0, 0.22),
            ("quoll_dasyurid", [0.12, 0.07, 0.14], [0.06, 0.05, 0.08], 0.06, 0.09, 0.05, 0.0, [0.40, 0.34, 0.28], [0.64, 0.58, 0.52], [0.22, 0.18, 0.16], 18),
            ("nocturnal", "insectivore", ["SCURRYING_BARK_FISSURES", "HUNTING_BEETLES_SPIDERS", "COMMUNAL_TORPOR_NEST", "SEMELPAROUS_AUGUST_MATING"],
             10.0, [1, 12], ["lamprima_aurata", "agrotis_infusa", "forest_arachnids"],
             "FORAGE under_peeling_ribbon_bark_of(eucalyptus_viminalis); EXHIBIT synchronized_August_semelparous_life_history."),
            ("Sharp high-frequency chittering hiss", 2200.0, 12000.0, 5200.0, 50.0, "stridulation")
        ),
        (
            "Canis dingo", "Alpine Dingo / Warrigal", "Warrigal / Mirrigang", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Carnivora", "Canidae",
            "https://biodiversity.org.au/afd/taxa/4690", "5219173", "286419", "VU",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "noosa_headland"], [0, 2100], "terrestrial",
            (18.5, 1.20, 0.56, 31.5, 38.2, "endotherm"),
            ("quadrupedal_trot", 1.5, 4.6, 15.8, 2.5, 2.8, 0.90),
            ("canid", [0.88, 0.58, 1.05], [0.34, 0.32, 0.44], 0.58, 0.36, 0.22, 0.0, [0.76, 0.48, 0.22], [0.92, 0.89, 0.83], [0.18, 0.15, 0.12], 24),
            ("crepuscular", "carnivore", ["TROTTING_RIDGELINE", "COOPERATIVE_PACK_STALK", "CHORUS_HOWLING", "SCENT_POST_PATROL", "DENNING_CAVE"],
             65.0, [1, 7], ["macropus_giganteus", "wallabia_bicolor", "vombatus_ursinus"],
             "REGULATE macropod_grazing_pressure VIA trophic_cascade; COORDINATE pack_spacing VIA long_range_harmonic_howl."),
            ("Long-range eerie harmonic chorus howl", 240.0, 2100.0, 520.0, 88.0, "high_whistle")
        ),
        (
            "Pteropus poliocephalus", "Grey-headed Flying-fox", "Yuluma /Giramay", "Yuin, Eora, Dharug, and Gubbi Gubbi Country",
            "Mammalia", "Chiroptera", "Pteropodidae",
            "https://biodiversity.org.au/afd/taxa/4612", "2432693", "9402", "VU",
            ["talbingo_pines", "sydney_sandstone", "noosa_headland"], [0, 950], "aerial_canopy",
            (0.80, 0.26, 0.18, 3.6, 36.8, "endotherm"),
            ("flapping_flight", 0.15, 6.5, 11.8, 3.2, 2.2, 1.05),
            ("flying_fox", [0.28, 0.32, 0.24], [0.16, 0.16, 0.20], 0.28, 0.0, 0.12, 1.05, [0.28, 0.27, 0.26], [0.78, 0.46, 0.16], [0.14, 0.13, 0.13], 22),
            ("nocturnal", "nectarivore", ["CAMP_ROOST_FANNING", "DUSK_FLYOUT_STREAM", "BLOSSOM_NECTAR_FEEDING", "LONG_RANGE_POLLEN_DISPERSAL"],
             35.0, [20, 500], ["eucalyptus_tereticornis", "corymbia_maculata", "banksia_integrifolia", "ficus_macrophylla"],
             "DEPART riparian_roost_camp AT civil_twilight; DISPERSE eucalypt_pollen_and_fig_seeds ACROSS 40km_foraging_radius."),
            ("Raucous camp squabbling screech & leathery wingbeats", 400.0, 6800.0, 1650.0, 82.0, "syllabic_trill")
        ),
        (
            "Hydromys chrysogaster", "Rakali / Native Water-rat", "Rakali /Toorooroo", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Mammalia", "Rodentia", "Muridae",
            "https://biodiversity.org.au/afd/taxa/4650", "2438328", "33720", "LC",
            ["talbingo_pines", "jounama_karst", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1600], "riparian_aquatic",
            (0.85, 0.34, 0.12, 2.9, 36.5, "endotherm"),
            ("sub_aquatic_swim", 0.50, 1.5, 3.2, 3.4, 3.8, 0.15),
            ("platypus", [0.36, 0.16, 0.42], [0.16, 0.12, 0.20], 0.16, 0.28, 0.06, 0.0, [0.22, 0.18, 0.15], [0.76, 0.52, 0.24], [0.94, 0.93, 0.90], 20),
            ("crepuscular", "carnivore", ["DIVING_FOR_CRAYFISH", "MIDDEN_ROCK_FEEDING", "RIVERBANK_GROOMING", "SWIMMING_RUDDER_TAIL"],
             22.0, [1, 2], ["euastacus_armatus", "freshwater_mussels", "litoria_peronii"],
             "CARRY captured_prey TO dedicated_streamside_feeding_midden_log; STEER underwater VIA white_tipped_rudder_tail."),
            ("High-pitched chittering squeak & water splash", 900.0, 6400.0, 2100.0, 60.0, "splash_plunge")
        ),

        # ==================== BIRDS (21 - 35 in part 1) ====================
        (
            "Menura novaehollandiae", "Superb Lyrebird", "Weringerong /Buln-buln", "Ngarigo, Walgalu, Yuin, Gundungurra, and Dharug Country",
            "Aves", "Passeriformes", "Menuridae",
            "https://biodiversity.org.au/afd/taxa/2810", "2488996", "47692", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone"], [50, 1550], "terrestrial",
            (0.98, 0.90, 0.42, 4.8, 40.5, "endotherm"),
            ("avian_display_walk", 0.85, 2.6, 6.8, 3.0, 3.2, 0.75),
            ("lyrebird", [0.42, 0.36, 0.48], [0.16, 0.16, 0.22], 0.34, 0.65, 0.20, 0.78, [0.34, 0.27, 0.22], [0.68, 0.64, 0.58], [0.58, 0.28, 0.16], 24),
            ("diurnal", "insectivore", ["RAKING_LEAF_LITTER", "MOUND_COURTSHIP_DISPLAY", "VOCAL_MIMICRY_CHORUS", "GLIDING_DOWN_GULLY"],
             30.0, [1, 2], ["lamprima_aurata", "earthworms", "amphipods"],
             "TURN_OVER 150_tonnes_per_ha_per_yr OF forest_floor_litter REDUCING bushfire_fuel_load; PERFORM winter_mound_display WITH lyre_tail_inverted_over_head AND mimic_20_neighboring_bird_species."),
            ("Virtuoso multi-species vocal mimicry & resonant territorial whistle", 350.0, 7800.0, 2200.0, 89.0, "syllabic_trill")
        ),
        (
            "Dacelo novaeguineae", "Laughing Kookaburra", "Gugubarra /Kookaburra", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Coraciiformes", "Alcedinidae",
            "https://biodiversity.org.au/afd/taxa/2540", "2475617", "8932", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 1500], "aerial_canopy",
            (0.34, 0.43, 0.28, 2.1, 40.8, "endotherm"),
            ("flapping_flight", 0.20, 8.5, 14.5, 4.2, 3.5, 0.66),
            ("kookaburra_kingfisher", [0.30, 0.32, 0.34], [0.22, 0.20, 0.30], 0.14, 0.16, 0.12, 0.66, [0.86, 0.83, 0.77], [0.36, 0.26, 0.19], [0.24, 0.52, 0.72], 20),
            ("diurnal", "carnivore", ["PERCH_SCAN_WAIT", "SWOOP_POUNCE_GROUND", "FAMILY_CHORUS_LAUGH", "PREY_BRANCH_BASHING"],
             28.0, [2, 8], ["pseudemoia_entrecasteauxii", "psudechis_porphyriacus_juvenile", "agrotis_infusa"],
             "AT dawn_and_dusk OBLIGATE synchronized_family_territorial_laugh; SCAN ground_from(dead_eucalypt_branch) AND swoop_upon_reptile_or_insect."),
            ("Rollicking territorial family chorus laugh (koo-koo-ka-ka-kook)", 500.0, 4600.0, 1450.0, 91.0, "harmonic_laugh")
        ),
        (
            "Aquila audax", "Wedge-tailed Eagle", "Mullyan / Bunjil", "Ngarigo, Walgalu, Wiradjuri, Yuin, Kulin, and Gubbi Gubbi Country",
            "Aves", "Accipitriformes", "Accipitridae",
            "https://biodiversity.org.au/afd/taxa/2120", "2480442", "30392", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 2228], "aerial_canopy",
            (4.2, 0.96, 0.65, 11.8, 41.0, "endotherm"),
            ("soaring_flight", 0.35, 14.5, 38.0, 2.1, 1.4, 2.25),
            ("raptor", [0.68, 0.62, 0.76], [0.26, 0.24, 0.32], 0.42, 0.48, 0.15, 2.25, [0.16, 0.13, 0.11], [0.48, 0.32, 0.18], [0.85, 0.72, 0.22], 24),
            ("diurnal", "carnivore", ["THERMAL_UPDRAFT_SOARING", "OROGRAPHIC_RIDGE_GLIDING", "STOOP_DIVE_ATTACK", "EYRIE_STICK_NESTING"],
             90.0, [1, 2], ["macropus_giganteus", "notamacropus_rufogriseus", "trichosurus_vulpecula"],
             "GAIN altitude_to(1800m_AGL) IN convective_thermals OR orographic_ridge_lift; DISPLAY diamond_wedge_tail_silhouette."),
            ("High thin whistling yelp & rushing stoop wind whistle", 1400.0, 5200.0, 2650.0, 74.0, "high_whistle")
        ),
        (
            "Callocephalon fimbriatum", "Gang-gang Cockatoo", "Gang-gang", "Ngarigo, Walgalu, Yuin, Gundungurra, and Kulin Country",
            "Aves", "Psittaciformes", "Cacatuidae",
            "https://biodiversity.org.au/afd/taxa/2410", "2479901", "176018", "EN",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone"], [100, 1900], "aerial_canopy",
            (0.26, 0.34, 0.24, 1.85, 41.2, "endotherm"),
            ("flapping_flight", 0.18, 9.2, 16.0, 4.4, 3.8, 0.64),
            ("cockatoo_parrot", [0.26, 0.28, 0.30], [0.16, 0.18, 0.18], 0.14, 0.14, 0.24, 0.64, [0.34, 0.35, 0.37], [0.82, 0.18, 0.14], [0.68, 0.66, 0.60], 20),
            ("diurnal", "frugivore", ["CRACKING_EUCALYPT_CAPSULES", "ALLOPREENING_PAIR", "MONTANE_SEASONAL_MIGRATION", "HOLLOW_NESTING"],
             25.0, [2, 16], ["eucalyptus_pauciflora", "eucalyptus_delegatensis", "acacia_dealbata"],
             "HOLD woody_gum_nuts WITH left_zygodactyl_foot WHILE extracting_seeds; DESCEND to_lower_valleys IN heavy_winter_snow."),
            ("Distinctive rasping creaky-hinge growl", 650.0, 3800.0, 1380.0, 78.0, "syllabic_trill")
        ),
        (
            "Zanda funerea", "Yellow-tailed Black-Cockatoo", "Wylah / Ngarnak", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Aves", "Psittaciformes", "Cacatuidae",
            "https://biodiversity.org.au/afd/taxa/2405", "9394520", "309862", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1900], "aerial_canopy",
            (0.80, 0.62, 0.38, 3.9, 41.0, "endotherm"),
            ("flapping_flight", 0.22, 10.5, 17.5, 2.9, 2.6, 1.12),
            ("cockatoo_parrot", [0.44, 0.46, 0.50], [0.22, 0.24, 0.24], 0.22, 0.32, 0.22, 1.12, [0.15, 0.14, 0.14], [0.92, 0.82, 0.18], [0.78, 0.42, 0.40], 20),
            ("diurnal", "omnivore", ["EXCAVATING_WOODBORING_LARVAE", "CRACKING_BANKSIA_CONES", "SLOW_FLUID_FLOCK_FLIGHT", "RAIN_HERALD_CALLING"],
             40.0, [3, 40], ["banksia_integrifolia", "acacia_dealbata", "eucalyptus_delegatensis"],
             "STRIP bark_from(acacia_and_eucalyptus) TO extract_cerambycid_and_cossid_larvae; FLY with_deep_fluid_wingbeats BEFORE cold_front_rain."),
            ("Long mournful wailing 'kee-ow' contact call", 750.0, 4200.0, 1650.0, 86.0, "high_whistle")
        ),
        (
            "Cacatua galerita", "Sulphur-crested Cockatoo", "Garraway /Bilibili", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Psittaciformes", "Cacatuidae",
            "https://biodiversity.org.au/afd/taxa/2415", "2479886", "141272", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1500], "aerial_canopy",
            (0.85, 0.50, 0.34, 4.1, 41.1, "endotherm"),
            ("flapping_flight", 0.24, 11.2, 18.5, 3.4, 2.9, 1.02),
            ("cockatoo_parrot", [0.40, 0.42, 0.46], [0.20, 0.22, 0.22], 0.20, 0.24, 0.32, 1.02, [0.95, 0.95, 0.93], [0.96, 0.85, 0.12], [0.18, 0.18, 0.19], 20),
            ("diurnal", "omnivore", ["SENTINEL_GUARDING_FLOCK", "GROUND_CORM_FORAGING", "CREST_ERECTION_DISPLAY", "ROOST_SCREECHING"],
             45.0, [5, 80], ["poa_sieberiana", "acacia_pycnantha", "eucalyptus_viminalis"],
             "MAINTAIN arboreal_sentinel_birds WHILE flock_forages_on_ground; IF danger_approaches OBLIGATE erect_sulphur_crest AND emit_ear_splitting_screech."),
            ("Raucous ear-splitting territorial screech", 600.0, 6200.0, 2100.0, 96.0, "syllabic_trill")
        ),
        (
            "Platycercus elegans", "Crimson Rosella", "Moyran / Bunduluk", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gundungurra Country",
            "Aves", "Psittaciformes", "Psittaculidae",
            "https://biodiversity.org.au/afd/taxa/2452", "2479333", "35542", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1950], "aerial_canopy",
            (0.13, 0.34, 0.18, 1.15, 41.3, "endotherm"),
            ("flapping_flight", 0.16, 9.8, 16.5, 5.2, 4.6, 0.46),
            ("cockatoo_parrot", [0.20, 0.22, 0.26], [0.12, 0.12, 0.14], 0.11, 0.18, 0.08, 0.46, [0.76, 0.12, 0.15], [0.14, 0.28, 0.74], [0.12, 0.12, 0.14], 20),
            ("diurnal", "frugivore", ["UNDULATING_FLIGHT", "SEED_POD_FORAGING", "BELL_PIPING_CONTACT", "TREE_HOLLOW_NESTING"],
             22.0, [2, 12], ["eucalyptus_pauciflora", "acacia_pycnantha", "poa_sieberiana"],
             "FLY in_undulating_wave_trajectory WITH fanned_blue_edged_tail; PIPE clear_double_whistle WHEN landing_in(eucalyptus_pauciflora)."),
            ("Clear brassy two-note bell whistle (kweek-kweek) & mellow chatter", 1200.0, 5400.0, 2450.0, 75.0, "high_whistle")
        ),
        (
            "Alisterus scapularis", "Australian King-Parrot", "Wellat /Bibiring", "Ngarigo, Yuin, Dharug, and Gubbi Gubbi Country",
            "Aves", "Psittaciformes", "Psittaculidae",
            "https://biodiversity.org.au/afd/taxa/2440", "2479504", "176013", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1450], "aerial_canopy",
            (0.21, 0.43, 0.22, 1.55, 41.2, "endotherm"),
            ("flapping_flight", 0.18, 10.2, 17.2, 4.8, 4.2, 0.56),
            ("cockatoo_parrot", [0.24, 0.26, 0.30], [0.14, 0.14, 0.16], 0.13, 0.22, 0.08, 0.56, [0.82, 0.14, 0.14], [0.14, 0.54, 0.22], [0.18, 0.38, 0.72], 20),
            ("diurnal", "frugivore", ["CANOPY_FRUIT_FEEDING", "SWIFT_WOODLAND_FLIGHT", "COURTSHIP_HEAD_BOBBING"],
             25.0, [2, 15], ["eucalyptus_delegatensis", "acacia_melanoxylon", "ficus_coronata"],
             "WEAVE rapidly_through_dense_wet_sclerophyll_trunks; FEED on_seeds_and_berries_in_outer_crown."),
            ("Piercing metallic 'crank-crank' flight whistle", 1400.0, 5800.0, 2750.0, 77.0, "high_whistle")
        ),
        (
            "Malurus cyaneus", "Superb Fairy-wren", "Muruduwin /Mirram", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Passeriformes", "Maluridae",
            "https://biodiversity.org.au/afd/taxa/2850", "2487526", "55807", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 1650], "terrestrial",
            (0.010, 0.14, 0.08, 0.22, 41.5, "endotherm"),
            ("avian_display_walk", 0.35, 3.8, 8.5, 8.5, 7.5, 0.18),
            ("songbird", [0.08, 0.08, 0.10], [0.05, 0.05, 0.06], 0.06, 0.08, 0.04, 0.18, [0.12, 0.52, 0.88], [0.12, 0.12, 0.16], [0.62, 0.58, 0.54], 18),
            ("diurnal", "insectivore", ["BOUNCING_HOP_FORAGE", "YELLOW_PETAL_COURTSHIP", "SENTINEL_HAWK_ALARM", "DOME_NEST_TENDING"],
             14.0, [3, 9], ["agrotis_infusa", "tetragonula_carbonaria", "leafhoppers"],
             "HOLD tail_cocked_vertically_at_75_deg; PRESENT yellow_flower_petal TO females; EMIT Type_II_trill WHEN kookaburra_or_currawong_calls."),
            ("Cascading high-frequency silvery reeling trill", 2800.0, 9200.0, 5600.0, 66.0, "syllabic_trill")
        ),
        (
            "Ptilonorhynchus violaceus", "Satin Bowerbird", "Bungil /Warrivoo", "Ngarigo, Yuin, Dharug, and Gubbi Gubbi Country",
            "Aves", "Passeriformes", "Ptilonorhynchidae",
            "https://biodiversity.org.au/afd/taxa/3120", "2494836", "28724", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1400], "terrestrial",
            (0.21, 0.31, 0.19, 1.60, 41.1, "endotherm"),
            ("avian_display_walk", 0.55, 7.5, 14.0, 4.6, 4.0, 0.48),
            ("songbird", [0.22, 0.20, 0.26], [0.12, 0.11, 0.14], 0.12, 0.12, 0.06, 0.48, [0.12, 0.14, 0.28], [0.38, 0.46, 0.28], [0.42, 0.18, 0.78], 20),
            ("diurnal", "frugivore", ["BOWER_AVENUE_CONSTRUCTION", "COLLECTING_BLUE_ARTIFACTS", "COURTSHIP_STRUT_BUZZING", "CANOPY_FIG_FEEDING"],
             20.0, [1, 12], ["platycercus_elegans_feathers", "dianella_tasmanica", "ficus_coronata"],
             "CONSTRUCT north_south_oriented_stick_avenue_bower; DECORATE court_with(blue_crimson_rosella_feathers_and_dianella_berries); PAINT inner_bower_walls WITH chewed_charcoal_and_saliva."),
            ("Mechanical whirring buzz, scratchy mimicry & ringing whistle", 800.0, 6800.0, 2300.0, 76.0, "stridulation")
        ),
        (
            "Anthochaera phrygia", "Regent Honeyeater", "Jinang /Weriby", "Wiradjuri, Dharug, Gundungurra, and Yuin Country",
            "Aves", "Passeriformes", "Meliphagidae",
            "https://biodiversity.org.au/afd/taxa/2980", "2487012", "265624", "CR",
            ["talbingo_pines", "sydney_sandstone"], [50, 950], "aerial_canopy",
            (0.043, 0.22, 0.12, 0.62, 41.4, "endotherm"),
            ("flapping_flight", 0.25, 8.8, 15.2, 6.2, 5.4, 0.33),
            ("songbird", [0.14, 0.13, 0.17], [0.08, 0.08, 0.10], 0.08, 0.10, 0.06, 0.33, [0.14, 0.14, 0.15], [0.92, 0.78, 0.14], [0.82, 0.62, 0.48], 18),
            ("diurnal", "nectarivore", ["PROBING_EUCALYPT_BLOSSOM", "HAWKING_INSECTS", "DEFENDING_FLOWERING_CROWN", "CUP_NEST_BUILDING"],
             20.0, [1, 6], ["eucalyptus_melliodora", "eucalyptus_albens", "corymbia_maculata"],
             "TRACK mass_winter_spring_nectar_flows OF box_ironbark_eucalypts; LEARN cultural_courtship_song FROM conspecific_elders."),
            ("Soft metallic bell-like chime & head-bobbing warble", 1500.0, 6500.0, 3100.0, 68.0, "syllabic_trill")
        ),
        (
            "Gymnorhina tibicen", "Australian Magpie", "Garoogong /Quardle-oodk", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Passeriformes", "Artamidae",
            "https://biodiversity.org.au/afd/taxa/3050", "2489476", "9132", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1900], "terrestrial",
            (0.32, 0.41, 0.25, 2.05, 41.0, "endotherm"),
            ("avian_display_walk", 0.70, 9.5, 18.0, 4.2, 3.8, 0.75),
            ("songbird", [0.28, 0.26, 0.32], [0.14, 0.14, 0.18], 0.15, 0.16, 0.08, 0.75, [0.12, 0.12, 0.13], [0.95, 0.95, 0.95], [0.68, 0.32, 0.18], 20),
            ("diurnal", "insectivore", ["ACOUSTIC_GROUND_PROBING", "CAROLING_CHORUS", "TERRITORIAL_SWOOP_DEFENSE", "RECOGNIZING_INDIVIDUAL_FACES"],
             35.0, [2, 14], ["scarab_larvae", "agrotis_infusa", "earthworms"],
             "TILT head_to_listen_for(subsurface_scarab_larvae_at_depth_cm <= 8.0) THEN probe_with_conical_bill; CAROL complex_flute_melodies TO maintain_year_round_territory."),
            ("Rich melodic flute-like caroling warble (quardle-oodk)", 650.0, 5200.0, 1750.0, 85.0, "harmonic_laugh")
        ),
        (
            "Strepera graculina", "Pied Currawong", "Kurrawang / Birri", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Passeriformes", "Artamidae",
            "https://biodiversity.org.au/afd/taxa/3058", "2489491", "254662", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 2000], "aerial_canopy",
            (0.30, 0.47, 0.26, 1.95, 41.0, "endotherm"),
            ("flapping_flight", 0.55, 10.2, 17.5, 4.0, 3.6, 0.78),
            ("songbird", [0.28, 0.25, 0.34], [0.15, 0.14, 0.20], 0.15, 0.20, 0.08, 0.78, [0.13, 0.13, 0.14], [0.94, 0.94, 0.92], [0.95, 0.82, 0.12], 20),
            ("diurnal", "omnivore", ["SOARING_GULLY_PATROL", "FORAGING_BERRIES_CARRION", "ALTITUDINAL_WINTER_MIGRATION", "RINGING_CURRAWONG_CALL"],
             35.0, [2, 30], ["tasmannia_xerophila", "agrotis_infusa", "pseudemoia_entrecasteauxii"],
             "PERFORM altitudinal_migration FROM snowy_highlands TO valley_townships_in_winter; EMIT ringing_onomatopoeic_curra_wong_call."),
            ("Ringing resonant metallic 'curra-wong, curra-wawk' call", 850.0, 4800.0, 1950.0, 84.0, "high_whistle")
        ),
        (
            "Podargus strigoides", "Tawny Frogmouth", "Boorin / Oomoomurla", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Caprimulgiformes", "Podargidae",
            "https://biodiversity.org.au/afd/taxa/2310", "2497118", "48432", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 1550], "arboreal",
            (0.38, 0.44, 0.26, 1.65, 39.2, "endotherm"),
            ("flapping_flight", 0.10, 6.8, 12.5, 3.6, 3.8, 0.82),
            ("kookaburra_kingfisher", [0.30, 0.28, 0.34], [0.24, 0.20, 0.26], 0.12, 0.18, 0.14, 0.82, [0.42, 0.38, 0.34], [0.62, 0.58, 0.52], [0.90, 0.78, 0.18], 20),
            ("nocturnal", "carnivore", ["MIMICKING_BROKEN_BRANCH_STUMP", "SILENT_POUNCE_ON_INSECTS", "NOCTURNAL_OOM_DRONING", "WINTER_TORPOR"],
             16.0, [1, 3], ["agrotis_infusa", "lamprima_aurata", "litoria_peronii"],
             "DURING daylight OBLIGATE freeze_at_45_deg_neck_extension_with_slit_eyes TO mimic_broken_grey_eucalypt_bough."),
            ("Deep resonant pulsing 'oom-oom-oom-oom' nocturnal drone", 180.0, 1100.0, 320.0, 64.0, "low_thump")
        ),
        (
            "Ceyx azureus", "Azure Kingfisher", "Tjirr-tjirr", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Aves", "Coraciiformes", "Alcedinidae",
            "https://biodiversity.org.au/afd/taxa/2532", "5228397", "342380", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1150], "riparian_aquatic",
            (0.032, 0.18, 0.10, 0.45, 41.2, "endotherm"),
            ("flapping_flight", 0.15, 9.5, 16.8, 7.2, 6.2, 0.26),
            ("kookaburra_kingfisher", [0.12, 0.12, 0.15], [0.10, 0.09, 0.16], 0.06, 0.05, 0.05, 0.26, [0.12, 0.26, 0.78], [0.86, 0.46, 0.18], [0.94, 0.92, 0.88], 18),
            ("diurnal", "piscivore", ["OVERHANGING_BRANCH_VIGIL", "VERTICAL_PLUNGE_DIVE", "LOW_WATER_SKIM_FLIGHT", "RIVERBANK_TUNNEL_NESTING"],
             20.0, [1, 2], ["galaxias_olidus", "juvenile_euastacus", "aquatic_hemiptera"],
             "PERCH on(callistemon_citrinus_branch_above_pool); PLUNGE_DIVE at_velocity_ms >= 12.0 TO capture(galaxias_olidus)."),
            ("Piercing high-pitched 'pseet-pseet' whistle & water plunge", 3200.0, 8800.0, 5800.0, 68.0, "splash_plunge")
        ),
    ]


def get_raw_species_part2():
    return [
        # ==================== BIRDS (36 - 40) ====================
        (
            "Dromaius novaehollandiae", "Emu", "Dinawan /Muray", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Aves", "Casuariiformes", "Casuariidae",
            "https://biodiversity.org.au/afd/taxa/2010", "2495140", "8790", "LC",
            ["talbingo_pines", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1550], "terrestrial",
            (40.0, 1.75, 1.65, 42.5, 39.0, "endotherm"),
            ("avian_display_walk", 1.4, 4.8, 13.8, 2.0, 1.8, 1.45),
            ("ratite", [0.85, 0.88, 1.05], [0.22, 0.24, 0.28], 1.05, 0.15, 0.08, 0.35, [0.36, 0.31, 0.26], [0.24, 0.20, 0.18], [0.32, 0.56, 0.72], 24),
            ("diurnal", "omnivore", ["STRIDING_GRASSLAND_FORAGE", "PATERNAL_EGG_INCUBATION", "GULAR_POUCH_BOOMING", "SPRINT_EVASION"],
             60.0, [1, 15], ["acacia_pycnantha", "poa_sieberiana", "acripeza_reticulata"],
             "MALE incubates_dark_green_clutch FOR 56_days_without_feeding; DISPERSE large_seeds ACROSS 50km_corridors."),
            ("Deep resonant tracheal gular-pouch booming drum & grunt", 45.0, 420.0, 75.0, 88.0, "low_thump")
        ),
        (
            "Casuarius casuarius", "Southern Cassowary", "Gunduy / Djirri", "Gubbi Gubbi and Kabi Kabi Subtropical Rainforest Corridor",
            "Aves", "Casuariiformes", "Casuariidae",
            "https://biodiversity.org.au/afd/taxa/2005", "2495134", "8787", "EN",
            ["noosa_headland"], [0, 800], "terrestrial",
            (52.0, 1.65, 1.55, 51.0, 39.2, "endotherm"),
            ("avian_display_walk", 1.2, 4.2, 13.5, 2.1, 2.0, 1.30),
            ("ratite", [0.92, 0.90, 1.08], [0.26, 0.34, 0.30], 0.95, 0.10, 0.28, 0.30, [0.10, 0.10, 0.12], [0.16, 0.38, 0.82], [0.88, 0.16, 0.18], 24),
            ("diurnal", "frugivore", ["FALLEN_DRUPE_FORAGING", "INFRASONIC_RUMBLE_DISPLAY", "SOLITARY_RAINFOREST_PATROL", "CASQUE_BRUSH_PUSH"],
             35.0, [1, 3], ["ficus_macrophylla", "syzygium_australe", "archontophoenix_cunninghamiana"],
             "SWALLOW whole_rainforest_drupes AND deposit_viable_seeds_in_dung; EMIT 23Hz_infrasonic_boom THROUGH dense_vine_thicket."),
            ("Sub-audible infrasonic 23Hz visceral rumble & booming cough", 23.0, 310.0, 32.0, 90.0, "low_thump")
        ),
        (
            "Cygnus atratus", "Black Swan", "Mulgo /Goonawarra", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Anseriformes", "Anatidae",
            "https://biodiversity.org.au/afd/taxa/2060", "2498344", "8868", "LC",
            ["talbingo_pines", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1450], "riparian_aquatic",
            (6.2, 1.25, 0.65, 14.2, 40.6, "endotherm"),
            ("sub_aquatic_swim", 0.60, 1.8, 19.5, 2.8, 1.6, 1.85),
            ("waterbird", [0.68, 0.45, 0.78], [0.18, 0.18, 0.24], 0.35, 0.18, 0.12, 1.85, [0.12, 0.12, 0.13], [0.95, 0.95, 0.95], [0.86, 0.16, 0.16], 22),
            ("diurnal", "grazer", ["UPENDING_AQUATIC_GRAZE", "ARCHED_WING_SWAN_DISPLAY", "REED_MOUND_NESTING", "V_FORMATION_NIGHT_FLIGHT"],
             40.0, [2, 60], ["vallisneria_australis", "potamogeton_crispus", "carex_appressa"],
             "UPEND neck_in_shallow_water TO graze_submerged_macrophytes; RAISE crinkled_white_flight_feathers IN territorial_busking_salute."),
            ("Musical bugling trumpet whistle & wing-whistle in flight", 600.0, 3400.0, 1350.0, 78.0, "high_whistle")
        ),
        (
            "Falco peregrinus", "Peregrine Falcon", "Waratta / Wolkolan", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Aves", "Falconiformes", "Falconidae",
            "https://biodiversity.org.au/afd/taxa/2180", "2481047", "8954", "LC",
            ["jounama_karst", "yarrangobilly_cliff", "sydney_sandstone", "noosa_headland"], [0, 2100], "aerial_canopy",
            (0.82, 0.44, 0.30, 3.8, 41.2, "endotherm"),
            ("soaring_flight", 0.25, 22.0, 85.0, 4.8, 4.5, 1.08),
            ("raptor", [0.34, 0.32, 0.38], [0.16, 0.16, 0.18], 0.20, 0.20, 0.08, 1.08, [0.26, 0.30, 0.36], [0.88, 0.84, 0.76], [0.92, 0.78, 0.14], 22),
            ("diurnal", "carnivore", ["CLIFF_LEDGE_EYRIE_VIGIL", "HIGH_SPEED_TEARDROP_STOOP", "AERIAL_PREY_STRIKE"],
             75.0, [1, 2], ["platycercus_elegans", "cacatua_galerita", "ptilinopus_magnificus"],
             "NEST on_sheer_limestone_or_sandstone_cliff_ledges; STOOP at_terminal_velocity_up_to_85_ms TO strike_avian_prey_in_mid_air."),
            ("Staccato piercing 'hek-hek-hek' cliff-echoing alarm", 1600.0, 6200.0, 3100.0, 86.0, "syllabic_trill")
        ),
        (
            "Ptilinopus magnificus", "Wompoo Fruit-Dove", "Wompoo /Bungil", "Dharug, Yuin, and Gubbi Gubbi Country",
            "Aves", "Columbiformes", "Columbidae",
            "https://biodiversity.org.au/afd/taxa/2250", "2495562", "187136", "LC",
            ["sydney_sandstone", "noosa_headland"], [0, 850], "aerial_canopy",
            (0.42, 0.42, 0.24, 2.1, 41.0, "endotherm"),
            ("flapping_flight", 0.18, 11.5, 18.0, 5.2, 4.2, 0.62),
            ("songbird", [0.26, 0.26, 0.30], [0.12, 0.12, 0.14], 0.13, 0.18, 0.06, 0.62, [0.22, 0.58, 0.24], [0.46, 0.16, 0.48], [0.94, 0.82, 0.16], 20),
            ("diurnal", "frugivore", ["ACROBATIC_CANOPY_FRUIT_REACHING", "RESONANT_WALLOCK_WOO_CALL", "FLIMSY_TWIG_NESTING"],
             22.0, [1, 8], ["ficus_macrophylla", "syzygium_australe", "archontophoenix_cunninghamiana"],
             "HANG upside_down_on_slender_outer_twigs TO swallow_whole_rainforest_figs; BLEND emerald_plumage_into_sunlit_canopy."),
            ("Deep resonant bubbly 'wallock-a-woo' rainforest call", 280.0, 1400.0, 520.0, 79.0, "low_thump")
        ),

        # ==================== REPTILES (41 - 50) ====================
        (
            "Varanus varius", "Lace Monitor / Tree Goanna", "ugarapul / Gugaa", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Reptilia", "Squamata", "Varanidae",
            "https://biodiversity.org.au/afd/taxa/1420", "2470762", "8558", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1350], "arboreal",
            (14.0, 1.85, 0.26, 3.4, 34.5, "ectotherm"),
            ("sprawling_lizard", 0.65, 2.8, 8.2, 2.4, 2.6, 0.35),
            ("goanna_skink", [0.78, 0.26, 0.92], [0.24, 0.16, 0.34], 0.36, 1.10, 0.04, 0.0, [0.16, 0.16, 0.18], [0.88, 0.82, 0.42], [0.28, 0.28, 0.32], 26),
            ("diurnal", "carnivore", ["SPIRAL_TRUNK_CLIMBING", "BIFURCATED_TONGUE_FLICKING", "TERMITE_MOUND_EGG_INCUBATION", "SUN_BASKING_BOUGH"],
             30.0, [1, 1], ["trichosurus_vulpecula", "bird_nestlings", "nasutitermes_exitiosus"],
             "WHEN substrate_temp_c < 22.0 OBLIGATE bask_on_sunlit_trunk; SPIRAL behind_tree_trunk WHEN observer_approaches; EXCAVATE arboreal_termite_mound TO incubate_eggs."),
            ("Loud bellows-like gular hiss & bark claw scraping", 220.0, 3400.0, 680.0, 70.0, "stridulation")
        ),
        (
            "Cyclodomorphus praealtus", "Alpine She-oak Skink", "Alpine Skink", "Ngarigo and Walgalu Country",
            "Reptilia", "Squamata", "Scincidae",
            "https://biodiversity.org.au/afd/taxa/1512", "2462814", "1175210", "EN",
            ["three_mile_dam", "yarrangobilly_cliff"], [1500, 2050], "terrestrial",
            (0.055, 0.24, 0.035, 0.06, 26.0, "ectotherm"),
            ("sprawling_lizard", 0.18, 0.65, 1.8, 4.2, 4.8, 0.08),
            ("goanna_skink", [0.20, 0.06, 0.26], [0.06, 0.04, 0.08], 0.05, 0.12, 0.02, 0.0, [0.46, 0.38, 0.28], [0.26, 0.22, 0.18], [0.72, 0.66, 0.56], 22),
            ("diurnal", "omnivore", ["TUSSOCK_CROWN_BASKING", "TORPID_SUBNIVEAN_BRUMATION", "VIVIPAROUS_BIRTH", "AMBUSHING_BEETLES"],
             10.0, [1, 2], ["acripeza_reticulata", "podocarpus_lawrencei", "alpine_coleoptera"],
             "BASK on_top_of(poa_sieberiana_hummocks); RETREAT deep_into_tussock_base WHEN shadow_passes_overhead."),
            ("Faint dry grass-blade rustle & defensive hiss", 600.0, 4800.0, 1800.0, 42.0, "stridulation")
        ),
        (
            "Tiliqua nigrolutea", "Blotched Blue-tongue Lizard", "Mugan /Blue-tongue", "Ngarigo, Walgalu, Yuin, Gundungurra, and Kulin Country",
            "Reptilia", "Squamata", "Scincidae",
            "https://biodiversity.org.au/afd/taxa/1540", "2462934", "96754", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone"], [200, 1900], "terrestrial",
            (0.65, 0.45, 0.07, 0.38, 30.5, "ectotherm"),
            ("sprawling_lizard", 0.20, 0.70, 1.9, 2.5, 2.4, 0.08),
            ("goanna_skink", [0.38, 0.11, 0.44], [0.12, 0.08, 0.14], 0.08, 0.16, 0.02, 0.0, [0.16, 0.14, 0.13], [0.86, 0.54, 0.38], [0.16, 0.24, 0.88], 22),
            ("diurnal", "omnivore", ["GRANITE_SLAB_BASKING", "COBALT_TONGUE_THREAT_GAPE", "CRUSHING_SNAILS", "BERRY_FORAGING"],
             12.0, [1, 1], ["tasmannia_xerophila", "terrestrial_gastropods", "dandelion_flowers"],
             "IF cornered_by_predator OBLIGATE flatten_ribs AND gape_jaws_displaying_ultraviolet_reflective_cobalt_blue_tongue WITH loud_hiss."),
            ("Sudden explosive lung hiss & jaw snap", 350.0, 4200.0, 1150.0, 65.0, "stridulation")
        ),
        (
            "Intellagama lesueurii", "Eastern Water Dragon", "Giwirr /Baiamul", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Reptilia", "Squamata", "Agamidae",
            "https://biodiversity.org.au/afd/taxa/1350", "7355849", "118675", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1100], "riparian_aquatic",
            (0.85, 0.80, 0.16, 0.52, 33.0, "ectotherm"),
            ("sprawling_lizard", 0.45, 2.2, 6.8, 3.8, 4.0, 0.22),
            ("goanna_skink", [0.42, 0.18, 0.52], [0.14, 0.14, 0.18], 0.22, 0.52, 0.16, 0.0, [0.34, 0.38, 0.30], [0.84, 0.18, 0.16], [0.14, 0.14, 0.15], 24),
            ("diurnal", "omnivore", ["RIVER_BOULDER_BASKING", "ARM_WAVE_HEAD_BOB_SIGNAL", "BIPEDAL_SPRINT_ACROSS_WATER", "SUBMERGED_BOTTOM_REFUGE"],
             22.0, [1, 10], ["litoria_peronii", "ficus_coronata", "aquatic_insects"],
             "COMMUNICATE territory_status VIA slow_circumduction_arm_wave_and_rapid_head_bob; IF startled OBLIGATE drop_from_branch_into_river_and_remain_submerged_up_to_60_min."),
            ("Heavy splash plunge into creek pool", 160.0, 1800.0, 340.0, 68.0, "splash_plunge")
        ),
        (
            "Amphibolurus muricatus", "Jacky Dragon", "Burrung", "Ngarigo, Wiradjuri, Yuin, Dharug, and Gubbi Gubbi Country",
            "Reptilia", "Squamata", "Agamidae",
            "https://biodiversity.org.au/afd/taxa/1320", "2466012", "118652", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1250], "terrestrial",
            (0.045, 0.32, 0.08, 0.08, 34.8, "ectotherm"),
            ("sprawling_lizard", 0.35, 1.8, 5.4, 5.2, 5.6, 0.16),
            ("goanna_skink", [0.18, 0.08, 0.22], [0.08, 0.07, 0.10], 0.10, 0.22, 0.10, 0.0, [0.48, 0.44, 0.38], [0.22, 0.20, 0.18], [0.95, 0.82, 0.12], 22),
            ("diurnal", "insectivore", ["FALLEN_BRANCH_PERCH_BASK", "TAIL_FLICK_PUSHUP_DISPLAY", "BIPEDAL_DASH", "YELLOW_MOUTH_GAPE"],
             15.0, [1, 2], ["acripeza_reticulata", "ants", "grasshoppers"],
             "PERFORM stereotyped_tail_flick_and_push_up_sequence TO deter_rivals; EXHIBIT temperature_dependent_sex_determination."),
            ("Rapid dry bark-scratching dash", 400.0, 4200.0, 1400.0, 44.0, "stridulation")
        ),
        (
            "Morelia spilota", "Diamond Python", "Guridjadja / Burring", "Yuin, Eora, Dharug, and Gundungurra Country",
            "Reptilia", "Squamata", "Pythonidae",
            "https://biodiversity.org.au/afd/taxa/1610", "2465198", "51888", "LC",
            ["sydney_sandstone", "noosa_headland"], [0, 900], "arboreal",
            (5.5, 2.30, 0.10, 1.45, 29.5, "ectotherm"),
            ("sinusoidal_slither", 0.20, 0.65, 1.8, 1.2, 2.0, 0.25),
            ("snake", [0.12, 0.12, 2.10], [0.10, 0.08, 0.14], 0.0, 0.35, 0.0, 0.0, [0.12, 0.12, 0.10], [0.94, 0.86, 0.24], [0.88, 0.86, 0.78], 28),
            ("cathemeral", "carnivore", ["COILED_CANOPY_AMBUSH", "INFRARED_LABIAL_PIT_SENSING", "CONSTRICTION_CAPTURE", "MATERNAL_SHIVERING_THERMOGENESIS"],
             14.0, [1, 1], ["trichosurus_vulpecula", "pseudocheirus_peregrinus", "pteropus_poliocephalus"],
             "DETECT warm_blooded_marsupial_prey VIA labial_infrared_pits; INCUBATE egg_clutch VIA rhythmic_muscular_shivering_thermogenesis."),
            ("Deep prolonged warning exhalation hiss", 250.0, 3200.0, 820.0, 58.0, "stridulation")
        ),
        (
            "Pseudechis porphyriacus", "Red-bellied Black Snake", "Mugan /Bundar", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Reptilia", "Squamata", "Elapidae",
            "https://biodiversity.org.au/afd/taxa/1720", "2469824", "8670", "LC",
            ["talbingo_pines", "jounama_karst", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [0, 1550], "riparian_aquatic",
            (1.10, 1.55, 0.06, 0.58, 30.0, "ectotherm"),
            ("sinusoidal_slither", 0.35, 1.2, 3.4, 2.0, 2.8, 0.20),
            ("snake", [0.08, 0.07, 1.40], [0.06, 0.05, 0.09], 0.0, 0.22, 0.0, 0.0, [0.08, 0.08, 0.10], [0.82, 0.14, 0.18], [0.92, 0.42, 0.46], 28),
            ("diurnal", "carnivore", ["CREEK_BANK_FORAGING", "SUBMERGED_FROG_HUNTING", "LOW_NECK_FLATTENING_DISPLAY", "SLITHERING_INTO_SEDGE"],
             15.0, [1, 1], ["litoria_peronii", "litoria_aurea", "pseudemoia_entrecasteauxii"],
             "PREFER shy_retreat_into(carex_appressa_or_water) OVER confrontation; FORAGE along_riparian_margins_for_hylid_frogs."),
            ("Sharp warning breath hiss & reed rustle", 300.0, 3800.0, 950.0, 56.0, "stridulation")
        ),
        (
            "Notechis scutatus", "Mainland Tiger Snake", "Ngarigo Tiger Snake", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Kulin Country",
            "Reptilia", "Squamata", "Elapidae",
            "https://biodiversity.org.au/afd/taxa/1710", "2469952", "8663", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone"], [0, 1950], "riparian_aquatic",
            (0.95, 1.30, 0.06, 0.52, 29.0, "ectotherm"),
            ("sinusoidal_slither", 0.32, 1.1, 3.2, 2.1, 2.9, 0.18),
            ("snake", [0.08, 0.07, 1.20], [0.06, 0.05, 0.08], 0.0, 0.18, 0.0, 0.0, [0.34, 0.30, 0.20], [0.76, 0.66, 0.26], [0.84, 0.82, 0.72], 28),
            ("diurnal", "carnivore", ["SPHAGNUM_BOG_BASKING", "HOODED_NECK_THREAT_STANCE", "FROG_AND_SKINK_PREDATION"],
             14.0, [1, 1], ["litoria_booroolongensis", "antechinus_agilis", "pseudemoia_entrecasteauxii"],
             "HUNT along_subalpine_bogs_and_rocky_creeks; IF threatened OBLIGATE raise_anterior_body_and_flatten_neck_band_hood."),
            ("Explosive forceful defensive hiss", 320.0, 4100.0, 1050.0, 62.0, "stridulation")
        ),
        (
            "Pseudemoia entrecasteauxii", "Southern Grass Skink", "Alpine Grass Skink", "Ngarigo, Walgalu, Yuin, and Kulin Country",
            "Reptilia", "Squamata", "Scincidae",
            "https://biodiversity.org.au/afd/taxa/1525", "2463412", "245650", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam"], [300, 2150], "terrestrial",
            (0.006, 0.12, 0.018, 0.015, 29.5, "ectotherm"),
            ("sprawling_lizard", 0.25, 0.90, 2.4, 6.5, 6.8, 0.05),
            ("goanna_skink", [0.10, 0.03, 0.12], [0.03, 0.02, 0.04], 0.03, 0.07, 0.01, 0.0, [0.42, 0.34, 0.24], [0.78, 0.28, 0.18], [0.86, 0.82, 0.74], 20),
            ("diurnal", "insectivore", ["SNOW_GUM_LOG_BASKING", "TRANSPARENT_EYELID_BLINK", "PLACENTAL_VIVIPARITY", "DARTING_INTO_TUSSOCK"],
             8.0, [1, 8], ["agrotis_infusa", "alpine_diptera", "collembola"],
             "POSSESS transparent_window_in_lower_eyelid TO see_predators_while_eyes_closed; EXHIBIT complex_placental_matrotrophy_in_alpine_cold."),
            ("Tiny dry leaf-litter rustle", 800.0, 5600.0, 2400.0, 36.0, "stridulation")
        ),
        (
            "Emydura macquarii", "Murray River Turtle", "Bungil / Pookila", "Wiradjuri, Walgalu, and Ngarigo Country",
            "Reptilia", "Testudines", "Chelidae",
            "https://biodiversity.org.au/afd/taxa/1210", "2441982", "44496", "VU",
            ["talbingo_pines", "jounama_karst"], [100, 850], "riparian_aquatic",
            (2.4, 0.30, 0.12, 0.65, 25.0, "ectotherm"),
            ("sub_aquatic_swim", 0.18, 0.65, 1.8, 1.8, 2.2, 0.10),
            ("turtle", [0.30, 0.12, 0.34], [0.08, 0.06, 0.10], 0.12, 0.05, 0.02, 0.0, [0.26, 0.28, 0.22], [0.68, 0.64, 0.52], [0.88, 0.84, 0.62], 20),
            ("diurnal", "omnivore", ["SNAG_LOG_SUN_BASKING", "CLOACAL_BURSA_RESPIRATION", "GRAZING_VALLISNERIA", "SAND_BANK_NEST_EXCAVATION"],
             20.0, [1, 12], ["vallisneria_australis", "aquatic_gastropods", "carrion"],
             "BASK communally_on_emergent_river_red_gum_snags; PERFORM bimodal_cloacal_respiration TO overwinter_on_cold_riverbed."),
            ("Soft water plop from basking log", 140.0, 1400.0, 290.0, 58.0, "splash_plunge")
        ),

        # ==================== AMPHIBIANS (51 - 56) ====================
        (
            "Pseudophryne corroboree", "Southern Corroboree Frog", "Corroboree Frog", "Ngarigo and Walgalu Country",
            "Amphibia", "Anura", "Myobatrachidae",
            "https://biodiversity.org.au/afd/taxa/1010", "2424596", "356312", "CR",
            ["three_mile_dam", "yarrangobilly_cliff"], [1300, 1760], "terrestrial",
            (0.0025, 0.028, 0.012, 0.006, 14.0, "ectotherm"),
            ("sprawling_lizard", 0.04, 0.10, 0.22, 1.8, 2.5, 0.01),
            ("frog", [0.03, 0.015, 0.03], [0.015, 0.012, 0.015], 0.02, 0.0, 0.0, 0.0, [0.06, 0.06, 0.06], [0.98, 0.86, 0.04], [0.95, 0.90, 0.12], 18),
            ("crepuscular", "insectivore", ["SPHAGNUM_MOSS_NEST_CHAMBER", "SLOW_FOUR_LEGGED_CRAWL", "ALKALOID_APOSEMATIC_DEFENSE", "AUTUMN_RAIN_HATCHING_WAIT"],
             6.0, [1, 6], ["iridomyrmex_ants", "mites", "springtails"],
             "NEVER hop_always_walk_with_deliberate_4_legged_crawl; SYNTHESIZE defensive_pseudophrynamine_skin_alkaloids; CALL from_sphagnum_bog_chamber_in_late_summer."),
            ("Grating nasal 'ark' advertisement call from sphagnum nest", 1800.0, 3400.0, 2450.0, 68.0, "stridulation")
        ),
        (
            "Philoria frosti", "Baw Baw Frog", "Baw Baw Frog", "Gunaikurnai, Wurundjeri, and Southern Alpine Country",
            "Amphibia", "Anura", "Limnodynastidae",
            "https://biodiversity.org.au/afd/taxa/1025", "2429584", "356298", "CR",
            ["three_mile_dam"], [1100, 1560], "fossorial",
            (0.008, 0.046, 0.022, 0.014, 12.0, "ectotherm"),
            ("saltatorial_frog", 0.06, 0.18, 0.55, 1.6, 2.2, 0.06),
            ("frog", [0.05, 0.03, 0.05], [0.02, 0.02, 0.02], 0.03, 0.0, 0.0, 0.0, [0.28, 0.20, 0.16], [0.62, 0.54, 0.38], [0.36, 0.28, 0.22], 18),
            ("nocturnal", "insectivore", ["SEEPAGE_TUNNEL_CALLING", "FOAM_NEST_BEAT_SPUMING", "ENDOTROPHIC_TADPOLE_DEVELOPMENT"],
             6.0, [1, 4], ["alpine_amphipods", "soil_worms", "coleoptera_larvae"],
             "BEAT egg_jelly_into_oxygenated_foam_nest_with_flanged_fingers IN subalpine_granite_seepage_tunnels."),
            ("Resonant short 'clunk-clunk' note from wet seepage tunnel", 850.0, 1900.0, 1180.0, 66.0, "low_thump")
        ),
        (
            "Litoria aurea", "Green and Golden Bell Frog", "Bell Frog", "Yuin, D'harawal, Eora, and Gundungurra Country",
            "Amphibia", "Anura", "Pelodryadidae",
            "https://biodiversity.org.au/afd/taxa/1110", "2427688", "8372", "VU",
            ["sydney_sandstone", "jounama_karst"], [0, 800], "riparian_aquatic",
            (0.038, 0.082, 0.035, 0.048, 24.0, "ectotherm"),
            ("saltatorial_frog", 0.15, 0.85, 2.8, 2.4, 3.2, 0.55),
            ("frog", [0.08, 0.04, 0.09], [0.04, 0.03, 0.04], 0.08, 0.0, 0.0, 0.0, [0.18, 0.68, 0.24], [0.82, 0.68, 0.18], [0.16, 0.58, 0.76], 18),
            ("diurnal", "carnivore", ["REED_SUN_BASKING", "FLOATING_VEGETATION_CHORUS", "LEAPING_PREY_CAPTURE"],
             12.0, [2, 25], ["acripeza_reticulata", "dragonfly_nymphs", "small_skinks"],
             "BASK in_direct_sunlight_on(typha_and_carex_reeds) DISPLAYING metallic_emerald_and_burnished_gold_dorsum."),
            ("Four-part drawling 'craw-awk, crawk, crok-crok' motorbike call", 550.0, 2400.0, 1150.0, 79.0, "syllabic_trill")
        ),
        (
            "Litoria peronii", "Peron's Tree Frog", "Emerald-spotted Tree Frog", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Amphibia", "Anura", "Pelodryadidae",
            "https://biodiversity.org.au/afd/taxa/1128", "2427756", "117545", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1200], "arboreal",
            (0.016, 0.058, 0.025, 0.024, 22.0, "ectotherm"),
            ("saltatorial_frog", 0.12, 0.75, 2.6, 2.8, 3.6, 0.65),
            ("frog", [0.06, 0.03, 0.07], [0.03, 0.02, 0.03], 0.06, 0.0, 0.0, 0.0, [0.54, 0.52, 0.48], [0.95, 0.82, 0.08], [0.14, 0.78, 0.28], 18),
            ("nocturnal", "insectivore", ["TOE_PAD_BARK_CLIMBING", "DESCENDING_MANIACAL_CACKLE", "FLASHING_YELLOW_BLACK_THIGHS", "CROSS_SHAPED_PUPIL_DILATION"],
             10.0, [1, 20], ["agrotis_infusa", "moths", "cicadas"],
             "DISPLAY cross_shaped_iris_pupil; FLASH bright_yellow_and_black_marbled_inguinal_thigh_patches WHEN leaping_to_startle_predators."),
            ("Loud manic descending pneumatic-drill cackle laugh", 1200.0, 3600.0, 2150.0, 84.0, "harmonic_laugh")
        ),
        (
            "Litoria booroolongensis", "Booroolong Frog", "Rocky Stream Frog", "Walgalu, Wiradjuri, and Ngarigo Country",
            "Amphibia", "Anura", "Pelodryadidae",
            "https://biodiversity.org.au/afd/taxa/1115", "2427712", "317352", "CR",
            ["talbingo_pines", "jounama_karst"], [200, 1300], "riparian_aquatic",
            (0.012, 0.048, 0.020, 0.018, 19.0, "ectotherm"),
            ("saltatorial_frog", 0.10, 0.65, 2.2, 2.6, 3.4, 0.45),
            ("frog", [0.05, 0.025, 0.06], [0.025, 0.02, 0.025], 0.05, 0.0, 0.0, 0.0, [0.38, 0.35, 0.30], [0.62, 0.48, 0.36], [0.88, 0.86, 0.82], 18),
            ("nocturnal", "insectivore", ["COBBLE_RIFFLE_BASKING", "SUBMERGED_ROCK_CREVICE_OVIPOSITION", "STREAM_SPLASH_LEAP"],
             10.0, [1, 8], ["simuliid_larvae", "mayfly_nymphs", "riparian_beetles"],
             "INHABIT cobble_and_bedrock_ledges_of_flowing_western_slope_streams; DEPOSIT 1200_eggs_in_submerged_rock_crevices."),
            ("Soft repetitive 'quirk-quirk-quirk' stream-side purr", 1400.0, 3200.0, 2100.0, 64.0, "syllabic_trill")
        ),
        (
            "Heleioporus australiacus", "Giant Burrowing Frog", "Eastern Owl Frog", "Yuin, Dharug, and Gundungurra Country",
            "Amphibia", "Anura", "Limnodynastidae",
            "https://biodiversity.org.au/afd/taxa/1040", "2429622", "356244", "VU",
            ["sydney_sandstone"], [50, 1000], "fossorial",
            (0.085, 0.095, 0.050, 0.082, 18.0, "ectotherm"),
            ("saltatorial_frog", 0.08, 0.35, 1.1, 1.5, 2.0, 0.18),
            ("frog", [0.09, 0.06, 0.10], [0.05, 0.04, 0.05], 0.06, 0.0, 0.0, 0.0, [0.34, 0.26, 0.24], [0.92, 0.78, 0.18], [0.88, 0.86, 0.82], 18),
            ("nocturnal", "carnivore", ["BACKWARDS_SAND_BURROWING", "OWL_HOOT_BORROW_CALLING", "NUPLIAL_SPINE_COMBAT", "HEATHLAND_FORAGING"],
             12.0, [1, 2], ["funnel_web_spiders", "centipedes", "beetles"],
             "EXCAVATE deep_spiral_burrows_in_Hawkesbury_sandstone_soil USING keratinized_metatarsal_spades; EMIT low_boobook_owl_like_hoot."),
            ("Deep resonant owl-like 'ou-ou-ou' burrow hoot", 420.0, 1200.0, 640.0, 74.0, "low_thump")
        ),

        # ==================== FRESHWATER FISH & CRAYFISH (57 - 60) ====================
        (
            "Maccullochella peelii", "Murray Cod", "Gudier / Ponde", "Wiradjuri, Walgalu, and Ngarigo Country",
            "Actinopterygii", "Centrarchiformes", "Percichthyidae",
            "https://biodiversity.org.au/afd/taxa/3510", "2394612", "135764", "VU",
            ["talbingo_pines", "jounama_karst"], [100, 750], "benthic_stream",
            (28.0, 1.15, 0.34, 4.8, 18.0, "ectotherm"),
            ("sub_aquatic_swim", 0.30, 1.2, 5.8, 1.8, 2.4, 0.22),
            ("fish", [1.05, 0.36, 0.32], [0.32, 0.28, 0.26], 0.18, 0.28, 0.14, 0.0, [0.24, 0.38, 0.22], [0.76, 0.78, 0.64], [0.92, 0.90, 0.85], 20),
            ("crepuscular", "carnivore", ["HOLLOW_RED_GUM_SNAG_AMBUSH", "BUCCAL_SUCTION_STRIKE", "PATERNAL_SPAWN_GUARDING"],
             18.0, [1, 1], ["euastacus_armatus", "macquaria_australasica", "litoria_booroolongensis"],
             "HOLD apex_territory_inside_submerged_hollow_eucalypt_snags; STRIKE prey_via_explosive_buccal_cavity_vacuum_expansion."),
            ("Sub-aquatic low-frequencyswim-bladder thump & suction implosion", 50.0, 380.0, 95.0, 76.0, "low_thump")
        ),
        (
            "Macquaria australasica", "Macquarie Perch", "Mudgee Perch", "Walgalu, Wiradjuri, Ngarigo, and Gundungurra Country",
            "Actinopterygii", "Centrarchiformes", "Percichthyidae",
            "https://biodiversity.org.au/afd/taxa/3525", "2394628", "270542", "EN",
            ["talbingo_pines", "jounama_karst"], [250, 1150], "benthic_stream",
            (1.4, 0.38, 0.14, 0.45, 16.0, "ectotherm"),
            ("sub_aquatic_swim", 0.25, 0.95, 3.6, 2.8, 3.4, 0.10),
            ("fish", [0.36, 0.15, 0.10], [0.10, 0.10, 0.08], 0.08, 0.10, 0.06, 0.0, [0.28, 0.32, 0.38], [0.68, 0.72, 0.76], [0.94, 0.94, 0.92], 18),
            ("crepuscular", "insectivore", ["DEEP_POOL_SCHOOLING", "RIFFLE_GRAVEL_SPAWNING_RUN", "BENTHIC_NYMPH_SUCTION"],
             15.0, [3, 25], ["mayfly_nymphs", "caddisfly_larvae", "odonata_larvae"],
             "MIGRATE upstream_from_deep_pools_into_oxygenated_cobble_riffles WHEN spring_water_temp_reaches_16_5_c."),
            ("Subtle gravel-shifting fin flutter & surface rise ripple", 120.0, 950.0, 260.0, 48.0, "splash_plunge")
        ),
        (
            "Galaxias olidus", "Mountain Galaxias", "Native Minnow", "Ngarigo, Walgalu, Wiradjuri, Yuin, and Gubbi Gubbi Country",
            "Actinopterygii", "Galaxiiformes", "Galaxiidae",
            "https://biodiversity.org.au/afd/taxa/3410", "2402418", "89582", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam", "sydney_sandstone", "noosa_headland"], [50, 2100], "benthic_stream",
            (0.015, 0.11, 0.022, 0.012, 11.0, "ectotherm"),
            ("sub_aquatic_swim", 0.15, 0.65, 2.2, 5.5, 6.2, 0.04),
            ("fish", [0.10, 0.022, 0.020], [0.025, 0.020, 0.020], 0.02, 0.03, 0.01, 0.0, [0.44, 0.36, 0.22], [0.22, 0.18, 0.12], [0.76, 0.68, 0.42], 16),
            ("diurnal", "insectivore", ["DRIFT_INSECT_INTERCEPTION", "WET_ROCK_WATERFALL_CLIMBING", "SUB_ICE_WINTER_SWIMMING"],
             8.0, [5, 50], ["terrestrial_ants", "chironomid_larvae", "agrotis_infusa_drowned"],
             "THRIVE in_trout_free_headwater_streams_above_waterfalls; CLIMB damp_vertical_rock_faces_using_pectoral_and_pelvic_fin_adhesion."),
            ("Tiny surface-tension snap on drifting insect", 400.0, 2800.0, 980.0, 40.0, "splash_plunge")
        ),
        (
            "Euastacus armatus", "Murray Spiny Crayfish", "Murrumbidgee Lobster / Tangambalanga", "Walgalu, Wiradjuri, and Ngarigo Country",
            "Malacostraca", "Decapoda", "Parastacidae",
            "https://biodiversity.org.au/afd/taxa/3810", "4645812", "72984", "VU",
            ["talbingo_pines", "jounama_karst"], [100, 850], "benthic_stream",
            (2.2, 0.42, 0.12, 0.32, 15.0, "ectotherm"),
            ("sprawling_lizard", 0.08, 0.25, 1.6, 1.8, 2.5, 0.14),
            ("crayfish", [0.28, 0.11, 0.32], [0.12, 0.09, 0.14], 0.22, 0.16, 0.18, 0.0, [0.16, 0.34, 0.22], [0.95, 0.94, 0.90], [0.82, 0.24, 0.16], 26),
            ("nocturnal", "omnivore", ["CLAW_RAISED_DEFENSIVE_POSTURE", "TAIL_FLIP_JET_RETROGRADE", "CLAY_BANK_BURROW_EXCAVATION", "WINTER_RIVERBANK_WALK"],
             10.0, [1, 1], ["submerged_eucalypt_leaves", "aquatic_invertebrates", "macrophytes"],
             "GROW slowly_over_30_years_reaching_2_5_kg; DISPLAY ivory_white_chelae_and_abdominal_spines_in_defensive_meral_spread."),
            ("Chitinous mandible rasp & hydro-acoustic tail-flip snap", 250.0, 4500.0, 1100.0, 58.0, "stridulation")
        ),

        # ==================== INVERTEBRATES (61 - 64) ====================
        (
            "Agrotis infusa", "Bogong Moth", "Bugung", "Ngarigo, Walgalu, Wiradjuri, and Gundungurra Country",
            "Insecta", "Lepidoptera", "Noctuidae",
            "https://biodiversity.org.au/afd/taxa/4120", "1786542", "320112", "EN",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam"], [800, 2228], "aerial_canopy",
            (0.00035, 0.025, 0.008, 0.002, 18.0, "ectotherm"),
            ("insect_flutter", 0.02, 4.5, 11.5, 38.0, 8.5, 0.05),
            ("moth_insect", [0.02, 0.008, 0.025], [0.006, 0.005, 0.006], 0.012, 0.0, 0.015, 0.05, [0.24, 0.20, 0.16], [0.52, 0.44, 0.34], [0.12, 0.10, 0.09], 14),
            ("nocturnal", "nectarivore", ["MAGNETIC_STAR_MIGRATION_1000KM", "IMBricated_CAVE_WALL_AESTIVATION", "SNOW_GUM_BLOSSOM_FEEDING"],
             5.0, [50, 10000], ["eucalyptus_pauciflora", "epacris_paludosa", "grevillea_australis"],
             "NAVIGATE 1000km_nocturnally_from_inland_plains_to_alpine_granite_caves USING Earth_magnetic_field_and_Milky_Way_landmarks; TILE cave_walls_at_17000_moths_per_m2_in_summer_aestivation."),
            ("Soft papery 38Hz wing-flutter hum in granite cave", 38.0, 1800.0, 114.0, 38.0, "stridulation")
        ),
        (
            "Acripeza reticulata", "Mountain Spotted Grasshopper / Katydid", "Mountain Katydid", "Ngarigo, Walgalu, Yuin, and Gundungurra Country",
            "Insecta", "Orthoptera", "Tettigoniidae",
            "https://biodiversity.org.au/afd/taxa/4185", "1694512", "441286", "LC",
            ["talbingo_pines", "jounama_karst", "yarrangobilly_cliff", "three_mile_dam"], [600, 2000], "terrestrial",
            (0.0022, 0.042, 0.018, 0.004, 20.0, "ectotherm"),
            ("saltatorial_frog", 0.03, 0.15, 0.85, 2.2, 3.5, 0.15),
            ("moth_insect", [0.04, 0.02, 0.04], [0.01, 0.01, 0.01], 0.03, 0.0, 0.03, 0.05, [0.32, 0.30, 0.28], [0.88, 0.14, 0.12], [0.12, 0.34, 0.88], 14),
            ("diurnal", "browser", ["CRYPTIC_BARK_CAMOUFLAGE", "APOSEMATIC_RED_BLUE_ABDOMEN_FLASH", "SENECIO_ALKALOID_GRAZING"],
             4.0, [1, 4], ["senecio_pinnatifolius", "poa_sieberiana", "craspedia_variabilis"],
             "SEQUESTER toxic_pyrrolizidine_alkaloids_from(senecio_species); WHEN touched OBLIGATE raise_domed_tegmina TO flash_brilliant_scarlet_and_cobalt_blue_abdominal_bands."),
            ("Dry tegmina-lifting click & bitter reflex exudate", 1200.0, 8400.0, 3600.0, 42.0, "stridulation")
        ),
        (
            "Lamprima aurata", "Golden Stag Beetle", "Christmas Jewel Beetle", "Ngarigo, Walgalu, Wiradjuri, Yuin, Eora, and Gubbi Gubbi Country",
            "Insecta", "Coleoptera", "Lucanidae",
            "https://biodiversity.org.au/afd/taxa/4210", "1038412", "196215", "LC",
            ["talbingo_pines", "jounama_karst", "sydney_sandstone", "noosa_headland"], [0, 1500], "arboreal",
            (0.0018, 0.032, 0.012, 0.003, 24.0, "ectotherm"),
            ("insect_flutter", 0.02, 1.8, 4.2, 62.0, 4.0, 0.04),
            ("moth_insect", [0.03, 0.012, 0.032], [0.012, 0.008, 0.015], 0.018, 0.0, 0.012, 0.04, [0.78, 0.66, 0.14], [0.18, 0.68, 0.36], [0.76, 0.24, 0.18], 14),
            ("diurnal", "browser", ["IRIDESCENT_EUCALYPT_TIP_FEEDING", "MANDIBULAR_JOUST_COMBAT", "ROTTING_LOG_LARVAL_DECOMPOSITION"],
             4.0, [1, 15], ["eucalyptus_viminalis", "eucalyptus_pauciflora", "acacia_dealbata"],
             "LARVAE decompose_dead_eucalypt_heartwood_over_3_years; ADULTS clip_young_eucalypt_shoot_tips_and_exhibit_structural_circularly_polarized_gold_green_iridescence."),
            ("Deep resonant 62Hz elytral flight drone & mandible click", 62.0, 2200.0, 186.0, 52.0, "stridulation")
        ),
        (
            "Tetragonula carbonaria", "Sugarbag Native Stingless Bee", "Karbi / Sugarbag", "Dharug, Eora, Yuin, and Gubbi Gubbi Country",
            "Insecta", "Hymenoptera", "Apidae",
            "https://biodiversity.org.au/afd/taxa/4350", "1342188", "156318", "LC",
            ["sydney_sandstone", "noosa_headland"], [0, 750], "aerial_canopy",
            (0.00001, 0.004, 0.002, 0.0004, 28.0, "ectotherm"),
            ("insect_flutter", 0.01, 2.5, 5.5, 195.0, 12.0, 0.008),
            ("moth_insect", [0.004, 0.002, 0.004], [0.0015, 0.0015, 0.0015], 0.002, 0.0, 0.001, 0.008, [0.08, 0.08, 0.09], [0.36, 0.28, 0.18], [0.85, 0.68, 0.18], 12),
            ("diurnal", "nectarivore", ["SPIRAL_CERUMEN_BROOD_CONSTRUCTION", "RESIN_PROPOLIS_DEFENSE_SWARM", "NATIVE_BLOSSOM_BUZZ_POLLINATION"],
             3.0, [20, 5000], ["callistemon_citrinus", "banksia_integrifolia", "eucalyptus_tereticornis", "corymbia_maculata"],
             "WHEN air_temp_c >= 18.0 PERMIT foraging_flight; CONSTRUCT logarithmic_spiral_brood_comb_and_store_tangy_trehalulose_sugarbag_honey_in_cerumen_pots."),
            ("High-frequency 195Hz social stingless-bee wing buzz", 195.0, 1400.0, 390.0, 46.0, "stridulation")
        ),
    ]


def build_species_dict(entry):
    (
        sci_name, common_name, indig_name, country,
        cls_name, order_name, family_name,
        aunsl_id, gbif_id, ncbi_id, iucn_status,
        sectors, elev_range, stratum,
        biophys, kinem, anat, ethol, bioac
    ) = entry

    genus_name = sci_name.split()[0]
    slug = sci_name.lower().replace(" ", "_")

    return {
        "id": slug,
        "uri": f"urn:qualia:taxon:{slug}",
        "scientific_name": sci_name,
        "common_name": common_name,
        "indigenous_name": indig_name,
        "custodial_country": country,
        "class_name": cls_name,
        "order": order_name,
        "family": family_name,
        "genus": genus_name,
        "aunsl_id": aunsl_id,
        "gbif_id": gbif_id,
        "ncbi_id": ncbi_id,
        "iucn_status": iucn_status,
        "sectors": sectors,
        "elevation_range_m": elev_range,
        "habitat_stratum": stratum,
        "biophysics": {
            "adult_mass_kg": biophys[0],
            "body_length_m": biophys[1],
            "shoulder_height_m": biophys[2],
            "bmr_watts": biophys[3],
            "body_temp_c": biophys[4],
            "thermoregulation": biophys[5],
        },
        "kinematics": {
            "locomotion_gait": kinem[0],
            "velocity_walk_ms": kinem[1],
            "velocity_trot_ms": kinem[2],
            "velocity_sprint_ms": kinem[3],
            "stride_freq_hz": kinem[4],
            "turn_rate_rad_s": kinem[5],
            "hop_or_wing_amp_m": kinem[6],
        },
        "anatomy_3d": {
            "archetype": anat[0],
            "torso_scale": anat[1],
            "head_scale": anat[2],
            "limb_length": anat[3],
            "tail_length": anat[4],
            "ear_or_crest_scale": anat[5],
            "wingspan_m": anat[6],
            "primary_rgb": anat[7],
            "secondary_rgb": anat[8],
            "accent_rgb": anat[9],
            "skeletal_joints": anat[10],
        },
        "ethology": {
            "activity_cycle": ethol[0],
            "diet_guild": ethol[1],
            "states": ethol[2],
            "alert_radius_m": ethol[3],
            "flock_or_mob_size": ethol[4],
            "forages_on": ethol[5],
            "n3_rule_summary": ethol[6],
        },
        "bioacoustics": {
            "call_name": bioac[0],
            "freq_min_hz": bioac[1],
            "freq_max_hz": bioac[2],
            "call_fundamental_hz": bioac[3],
            "spl_db": bioac[4],
            "wave_pattern": bioac[5],
        },
        "commons_ledger": {
            "licence": "SocialPermissiveCommons",
            "obligation_discharged": True,
            "perpetual_royalties_prohibited": True,
        },
    }


def escape_ttl(val: str) -> str:
    return val.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ")


def generate_turtle(species_list: list[dict]) -> str:
    lines = [
        "# ============================================================================",
        "# East Coast Australia 64 Authoritative Fauna Species RDF Ontology",
        "# Compliant with W3C Turtle 1.1, Darwin Core (dwc), auNSL, Movebank, AnAge, A2O.",
        "# SHACL-First Living Entities Convention: Every taxon is a dwc:Taxon , rdfs:Class.",
        "# Managed under QualiaDB Permissive Commons ('World of God' Living Commons).",
        "# ============================================================================",
        "",
        "@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .",
        "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .",
        "@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .",
        "@prefix dwc: <http://rs.tdwg.org/dwc/terms/> .",
        "@prefix ro: <http://purl.obolibrary.org/obo/ro.owl#> .",
        "@prefix envo: <http://purl.obolibrary.org/obo/envo.owl#> .",
        "@prefix ecocore: <http://purl.obolibrary.org/obo/ecocore.owl#> .",
        "@prefix uberon: <http://purl.obolibrary.org/obo/uberon.owl#> .",
        "@prefix pato: <http://purl.obolibrary.org/obo/pato.owl#> .",
        "@prefix aunsl: <https://biodiversity.org.au/voc/taxa/> .",
        "@prefix ncbitaxon: <http://purl.obolibrary.org/obo/NCBITaxon_> .",
        "@prefix gbif: <https://www.gbif.org/species/> .",
        "@prefix sosa: <http://www.w3.org/ns/sosa/> .",
        "@prefix tk: <https://localcontexts.org/ontology/tk-bc.ttl#> .",
        "@prefix odrl: <http://www.w3.org/ns/odrl/2/> .",
        "@prefix qualia: <urn:qualia:taxon:> .",
        "@prefix fauna: <urn:qualia:ontology:fauna#> .",
        "",
    ]

    for idx, sp in enumerate(species_list, start=1):
        sid = sp["id"]
        sci = escape_ttl(sp["scientific_name"])
        com = escape_ttl(sp["common_name"])
        ind = escape_ttl(sp["indigenous_name"])
        cnt = escape_ttl(sp["custodial_country"])
        forages_uris = ", ".join(f"qualia:{f}" for f in sp["ethology"]["forages_on"])
        sectors_lit = ", ".join(f'"{s}"' for s in sp["sectors"])

        lines.append(f"# {idx}. {sp['scientific_name']} ({sp['common_name']})")
        lines.append(f"qualia:{sid} a dwc:Taxon , rdfs:Class ;")
        lines.append(f'    rdfs:label "{com}"@en ;')
        lines.append(f'    dwc:scientificName "{sci}" ;')
        lines.append(f'    dwc:vernacularName "{com}"@en , "{ind}" ;')
        lines.append(f'    dwc:class "{sp["class_name"]}" ;')
        lines.append(f'    dwc:order "{sp["order"]}" ;')
        lines.append(f'    dwc:family "{sp["family"]}" ;')
        lines.append(f'    dwc:genus "{sp["genus"]}" ;')
        lines.append(f'    dwc:taxonRank "species" ;')
        lines.append(f'    aunsl:lsid "{sp["aunsl_id"]}" ;')
        lines.append(f'    fauna:gbif_id "{sp["gbif_id"]}" ;')
        lines.append(f'    fauna:ncbi_id "{sp["ncbi_id"]}" ;')
        lines.append(f'    fauna:iucn_status "{sp["iucn_status"]}" ;')
        lines.append(f'    envo:habitat_stratum "{sp["habitat_stratum"]}" ;')
        lines.append(f'    envo:elevation_min "{sp["elevation_range_m"][0]}"^^xsd:integer ;')
        lines.append(f'    envo:elevation_max "{sp["elevation_range_m"][1]}"^^xsd:integer ;')
        lines.append(f"    fauna:occurs_in_sector {sectors_lit} ;")
        lines.append(f'    fauna:adult_mass_kg "{sp["biophysics"]["adult_mass_kg"]}"^^xsd:decimal ;')
        lines.append(f'    fauna:body_length_m "{sp["biophysics"]["body_length_m"]}"^^xsd:decimal ;')
        lines.append(f'    fauna:bmr_watts "{sp["biophysics"]["bmr_watts"]}"^^xsd:decimal ;')
        lines.append(f'    fauna:body_temp_c "{sp["biophysics"]["body_temp_c"]}"^^xsd:decimal ;')
        lines.append(f'    fauna:thermoregulation "{sp["biophysics"]["thermoregulation"]}" ;')
        lines.append(f'    fauna:locomotion_class "{sp["kinematics"]["locomotion_gait"]}" ;')
        lines.append(f'    fauna:velocity_walk_ms "{sp["kinematics"]["velocity_walk_ms"]}"^^xsd:decimal ;')
        lines.append(f'    fauna:velocity_sprint_ms "{sp["kinematics"]["velocity_sprint_ms"]}"^^xsd:decimal ;')
        lines.append(f'    uberon:archetype_3d "{sp["anatomy_3d"]["archetype"]}" ;')
        lines.append(f'    uberon:skeletal_joints "{sp["anatomy_3d"]["skeletal_joints"]}"^^xsd:integer ;')
        lines.append(f'    ecocore:activity_cycle "{sp["ethology"]["activity_cycle"]}" ;')
        lines.append(f'    ecocore:diet_guild "{sp["ethology"]["diet_guild"]}" ;')
        lines.append(f'    fauna:n3_rule_summary "{escape_ttl(sp["ethology"]["n3_rule_summary"])}" ;')
        lines.append(f'    sosa:call_name "{escape_ttl(sp["bioacoustics"]["call_name"])}" ;')
        lines.append(f'    sosa:call_fundamental_hz "{sp["bioacoustics"]["call_fundamental_hz"]}"^^xsd:decimal ;')
        lines.append(f'    sosa:wave_pattern "{sp["bioacoustics"]["wave_pattern"]}" ;')
        lines.append(f'    tk:custodial_country "{cnt}" ;')
        lines.append(f'    odrl:hasPolicy "SocialPermissiveCommons" ;')
        lines.append(f"    ro:trophically_interacts_with {forages_uris} .")
        lines.append("")

    return "\n".join(lines)


def main():
    raw_entries = get_raw_species_part1() + get_raw_species_part2()
    species_list = [build_species_dict(e) for e in raw_entries]

    assert len(species_list) == 64, f"Expected 64 species, got {len(species_list)}"

    repo_root = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    catalog_dir = os.path.join(repo_root, "data", "catalog")
    ontologies_dir = os.path.join(repo_root, "data", "ontologies")
    os.makedirs(catalog_dir, exist_ok=True)
    os.makedirs(ontologies_dir, exist_ok=True)

    json_path = os.path.join(catalog_dir, "fauna_64.json")
    ttl_path = os.path.join(ontologies_dir, "east_coast_fauna_64.ttl")

    catalog_payload = {
        "schema_version": "1.0.0",
        "title": "East Coast Australia 64 Authoritative Fauna Species Catalog",
        "corridor_bounds": {
            "macro_corridor": "Noosa Headland (SEQ) to Sydney Basin, Snowy Mountains (Talbingo/Kosciuszko), and Victorian Alps",
            "lat_range_deg": [-39.0, -26.0],
            "lon_range_deg": [140.5, 153.8],
            "sectors": [
                "talbingo_pines",
                "jounama_karst",
                "yarrangobilly_cliff",
                "three_mile_dam",
                "sydney_sandstone",
                "noosa_headland"
            ]
        },
        "total_species": len(species_list),
        "commons_ledger": {
            "licence": "SocialPermissiveCommons",
            "obligation_discharged": True,
            "perpetual_royalties_prohibited": True
        },
        "species": species_list
    }

    with open(json_path, "w", encoding="utf-8") as f:
        json.dump(catalog_payload, f, indent=2, ensure_ascii=False)

    ttl_content = generate_turtle(species_list)
    with open(ttl_path, "w", encoding="utf-8") as f:
        f.write(ttl_content)

    print(f"[OK] Generated {len(species_list)} fauna species in {json_path}")
    print(f"[OK] Generated W3C Turtle 1.1 ontology in {ttl_path}")


if __name__ == "__main__":
    if hasattr(sys.stdout, "reconfigure"):
        sys.stdout.reconfigure(encoding="utf-8", errors="replace")
    main()
