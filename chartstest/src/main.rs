use charts::{BarLabelPosition, Chart, ScaleBand, ScaleLinear, VerticalBarView};

fn main() {
    let width = 2560;
    let height = 1440;
    let padding_top = 90;
    let padding_right = 40;
    let padding_bottom = 450;
    let padding_left = 60;

    let values = [
        ("channel395_0", 7.94f32),
        ("meshBlockPipe_0", 0.19),
        ("3DValve_0", 73.99),
        ("forwardStepPar_2", 35.22),
        ("pitzDaily_0", 4.75),
        ("cavityBox", 18.48),
        ("rightAngledDuctFlow_0", 6.00),
        ("BernardCells_0", 3.91),
        ("shockTube_0", 1.12),
        ("aerofoilNACA0012_0", 23.97),
        ("backwardFacingStep", 1.15),
        ("pollutantPlumePar_0", 14.85),
        ("KCS_Fn026Par_0", 143.88),
        ("acousticAnalogyPar_0", 99.13),
        ("trainTunnelPar_0", 202.25),
        ("angledDuctPorous_0", 9.52),
        ("wing", 54.87),
        ("2DMixer", 6.17),
        ("mixerVessel2D_1", 1.85),
        ("meshBlockSphere_0", 0.16),
        ("twoThrustersPar_0", 59.51),
        ("singleChannelPump_3", 91.62),
        ("2DcavityBlay_0", 1.01),
        ("hotRoom_3", 2.75),
        ("Channelf4st_p2_0", 89.80),
        ("2DRotatingBox_0", 15.49),
        ("2DcavityNielsen_0", 5.67),
        ("cylindricalDomainWindPar_3", 18.20),
        ("airFoil2D_0", 4.27),
        ("channelPorousBafflePressure_0", 2.82),
        ("2DCylinderPar_1", 12.63),
        ("fuelSloshing_0", 9.65),
        ("meshCavityPar_0", 1.28),
        ("irvineForest_0", 12.50),
        ("meshMotorBikePar_0", 19.19),
        ("meshAddLayersToFaceZone_0", 0.26),
        ("twoBladeMixerPar_0", 47.00),
        ("meshRelativeMotion_0", 6.90),
        ("boundaryWallFunctions_2", 0.64),
        ("2DCylinderPar_2", 7.61),
        ("pitzDaily_4", 3.99),
        ("lateralCavity_0", 1.27),
        ("rightAngledDuct", 54.09),
        ("channelWithLumpedBCs_0", 2.28),
        ("interfaceQadd_0", 11.35),
        ("porousBlockage_0", 2.11),
        ("pitzDailyCoupled_0", 7.01),
        ("windAroundBuildingsPar_0", 45.02),
        ("irvineForest_1", 12.35),
        ("cylindricalPeriodicSectors_0", 0.59),
        ("mixerVessel2D_0", 16.89),
        ("2DcavityBettsBokhari_0", 12.28),
        ("boundaryLaunderSharma_0", 0.78),
        ("rotorDisk_0", 39.93),
        ("meshGapDetectionPar_0", 0.83),
        ("underHoodCarChannel_0", 70.18),
        ("hotRoom_0", 6.12),
        ("resonantSloshing_0", 0.63),
        ("meshFilterPar_0", 1.84),
        ("pisoFoamCavity_0", 0.87),
        ("boundaryPatchSwitch_0", 66.24),
        ("2DManifold", 32.05),
        ("meshAirfoilPar_1", 13.19),
        ("nacaAirfoil_0", 11.31),
        ("meshPumpPar_0", 18.37),
        ("leafHeatBalance_0", 11.17),
        ("simpleCar_0", 4.59),
        ("hotRoom_2", 2.55),
        ("porousTubePar_0", 27.20),
        ("boundaryWallFunctions_1", 0.64),
        ("meshBlockSphere7ProjectedEdges_0", 0.24),
        ("injection_0", 49.68),
        ("ovoidRadiationPar_0", 38.14),
        ("rotatingHeatedDiscPar_0", 71.12),
        ("damBreakHydro_0", 1.95),
        ("cylindricalDomainWindPar_4", 51.92),
        ("jouleHeatingSolidPar_0", 24.75),
        ("fallingSphere_0", 85.80),
        ("porousDamBreak_0", 124.93),
        ("2DCavityTransient_0", 38.32),
        ("meshExtrudePipesPar_0", 12.78),
        ("planarContraction_0", 12.48),
        ("wing", 2.07),
        ("rotSector_0", 29.11),
        ("meshBlockSphere7_0", 0.24),
        ("shadowingTree_0", 14.20),
        ("hotMovingCone_0", 18.69),
        ("windProfilePar_0", 5.48),
        ("cylinderAdjustFlowUFixPar_1", 9.64),
        ("meshIglooWithFridges_0", 9.15),
        ("solidConductionBlock_0", 16.80),
        ("3DBox", 16.75),
        ("oneraM6Par_0", 49.84),
        ("nacaAirfoil_1", 8.89),
        ("pitzDailyTransient_0", 16.85),
        ("phaseSettling_0", 17.62),
        ("meshFlange_0", 7.85),
        ("sunDirModel_0", 10.29),
        ("brakeDisk_0", 62.53),
        ("buildingWithWindowsAndSolarPar_0", 33.42),
        ("rae2822Par_0", 10.08),
        ("meshAirfoilPar_2", 10.38),
        ("cylindricalDomainWindPar_2", 37.49),
        ("ovoidRadiationPar_1", 34.31),
        ("annularThermalMixer_0", 44.14),
        ("meshBox_0", 4.50),
        ("ovoidRadiationPar_3", 46.54),
        ("T3A_0", 18.72),
        ("fallingObject_0", 87.77),
        ("forwardStepPar_1", 38.35),
        ("electricHeatingElement_0", 38.05),
        ("boundaryWallFunctions_0", 1.48),
        ("rae2822_0", 27.98),
        ("objectFallingToWater_0", 92.84),
        ("singleChannelPump_0", 330.78),
        ("ovoidRadiationPar_2", 33.22),
        ("turbineSiting_0", 36.40),
        ("forwardStepPar_0", 28.51),
        ("cylinderAdjustFlowPFixPar_1", 9.62),
        ("cylinderAdjustFlowPFixPar_0", 10.63),
        ("cylinderAdjustFlowUFixPar_0", 10.92),
        ("cavityCoupledU_0", 0.79),
        ("ovoidHumidity_1", 45.55),
        ("singleChannelPump_2", 128.58),
        ("naca0012", 81.48),
        ("twoRegionAnisoSolid_0", 0.45),
        ("meshSnakeRiverCanyon_0", 3.78),
        ("2DRotatingBox_1", 16.66),
        ("linearSloshingPar_0", 233.94),
        ("meshWindTunnelRotationPar_0", 17.15),
        ("singleChannelPump_1", 100.90),
        ("backwardFacingStep", 13.66),
        ("cylindricalDomainWindPar_0", 48.59),
        ("solidConductionBlock_1", 7.51),
        ("Ahmed25deg_1", 76.20),
        ("reconstructWindProfilePar_0", 5.86),
        ("wingMotion2D_0", 19.89),
        ("meshAirfoilPar_0", 6.71),
        ("2DManifold", 1.80),
        ("2DCylinderPar_0", 13.26),
    ];

    let formatted = values
        .iter()
        .map(|(name, value)| (format!("{name:>40}"), *value))
        .collect::<Vec<_>>();

    let x_labels = ScaleBand::new()
        .set_domain(formatted.iter().map(|(name, _)| name.to_string()).collect())
        .set_range(vec![0, width - padding_left - padding_right]);
    let max = values
        .iter()
        .fold(f32::MIN, |current, incoming| current.max(incoming.1));
    let y_labels = ScaleLinear::new()
        .set_domain(vec![0f32, max])
        .set_range(vec![height - padding_top - padding_bottom, 0]);
    let view = VerticalBarView::new()
        .set_x_scale(&x_labels)
        .set_y_scale(&y_labels)
        .load_data(&formatted)
        .unwrap();

    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(padding_top, padding_right, padding_bottom, padding_left)
        .add_view(&view)
        .add_axis_bottom(&x_labels)
        .add_axis_left(&y_labels)
        .set_bottom_axis_tick_label_rotation(-90)
        .set_left_axis_tick_label_format(".2s")
        .save("graph.svg")
        .unwrap();
}
