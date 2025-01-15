use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use crate::save_data::{
    RcCell,
};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[serde(transparent)]
#[repr(transparent)]
pub struct WarAsset(pub i32);

impl Deref for WarAsset {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WarAsset {
    pub fn as_str(&self) -> &'static str {
        let id: i32 = self.0.into();
        let name = WAR_ASSET_NAMES.get(&id);
        name.unwrap_or(&"<Unknown>")
    }
}

impl From<i32> for WarAsset {
    fn from(value: i32) -> Self {
        WarAsset(value)
    }
}

impl From<WarAsset> for i32 {
    fn from(value: WarAsset) -> Self {
        value.0
    }
}

impl From<WarAsset> for RcCell<i32> {
    fn from(value: WarAsset) -> Self {
        Self::new(value.0)
    }
}

lazy_static! {
    static ref WAR_ASSET_NAMES: HashMap<i32, &'static str> = HashMap::from([
        (6,"TerminusFleet_Military"),
        (7,"BloodPackFlotilla_Modifier"),
        (8,"BlueSunsFlotilla_Modifier"),
        (9,"EclipseFlotilla_Modifier"),
        (52,"RachniiWorkers_Military"),
        (58,"VolusDreadnaughtKwunu_Military"),
        (76,"ShadowBrokerSupportTeam_Military"),
        (112,"ElcorFlotilla_Military"),
        (147,"ShadowBrokerWetSquad_Military"),
        (179,"SpectreTeam_Military"),
        (182,"VolusBombingFleet_Military"),
        (185,"HanarAndDrellForces_Military"),
        (248,"ImprovedHanarMedicalTech_Modifier"),
        (193,"BatarianFleet_Military"),
        (195,"CommanderKahairalBalak_Modifier"),
        (254,"PillarsOfStrengthBonus_Modifier"),
        (207,"CitadelDefenseForce_Military"),
        (225,"SupportedCitadelRefugees_Modifier"),
        (226,"CivilianMedicalVolunteers_Modifier"),
        (227,"ReassuredArguingCouple_Modifier"),
        (228,"AsariPatientSuicide_Modifier"),
        (229,"ImproveCivilianMorale_Modifier"),
        (231,"LoweredCrime_Modifier"),
        (232,"FewerRefugees_Modifier"),
        (234,"MedicalSuppliesReleased_Modifier"),
        (235,"IncreasedSurveillance_Modifier"),
        (236,"CivilianMilitia_Modifier"),
        (237,"IncreasedCrimeRate_Modifier"),
        (238,"GrissomStudentHousing_Modifier"),
        (240,"EnforceEveryLaw_Modifier"),
        (241,"CrackDownOnTerror_Modifier"),
        (242,"CivilianDonations_Modifier"),
        (245,"GethJammingFrequencies_Modifier"),
        (246,"CerberusTurretSchematics_Modifier"),
        (247,"ImprovedAsariAmps_Modifier"),
        (255,"BookOfPlenixBonus_Modifier"),
        (261,"CodeOfTheAncientsBonus_Modifier"),
        (262,"RingsOfAluneBonus_Modifier"),
        (0,"AllianceEngineeringCorp_Military"),
        (163,"BreederQueenBetrayal_Modifier"),
        (257,"ProtheanDataDrivesBonus_Modifier"),
        (259,"ObeliskOfKarzaBonus_Modifier"),
        (263,"HesperiaPeriodStatueBonus_Modifier"),
        (1,"103rdMarineDivision_Military"),
        (17,"AllianceSpecOpsTeamEcho_Modifier"),
        (126,"AllianceMarineRecon_Modifier"),
        (138,"BioticSupport_Modifier"),
        (140,"ArrivalNotCompleted_Modifier"),
        (220,"AntiCerberusInterview_Modifier"),
        (2,"AdmiralMikhailovich_Military"),
        (139,"SavedTheCouncilInME1_Modifier"),
        (3,"Alliance1stFleet_Military"),
        (16,"AllianceFrigateAgrincourt_Modifier"),
        (110,"AllianceFrigateLeipzig_Modifier"),
        (251,"ChemicalBurnTreatments_Modifier"),
        (4,"Alliance3rdFleet_Military"),
        (37,"AllianceCruiserLondon_Modifier"),
        (127,"AllianceFrigateTrafalger_Modifier"),
        (194,"SabotagedAllianceShips_Modifier"),
        (230,"AllianceFleetLosses_Modifier"),
        (190,"CerberusAttack_Modifier"),
        (28,"KhaleeSanders_Military"),
        (201,"TechStudentsRescued_Modifier"),
        (202,"SandersArcherUpgrade_Modifier"),
        (29,"BioticCompany_Military"),
        (30,"Jack_Military"),
        (63,"Arcturus1stDivision_Military"),
        (68,"AllianceSpecOpsTeamDelta_Military"),
        (69,"AllianceCruiserShanghai_Military"),
        (71,"NavalEngineeringFlotilla_Military"),
        (114,"CommnicationsArray_Military"),
        (128,"Alliance6thFleet_Military"),
        (131,"DrChakwas_Military"),
        (134,"RogueFighterSquadron_Military"),
        (135,"N7SpecialOps_External"),
        (108,"AllianceFrigateHongKong_Modifier"),
        (173,"Alliance5thFleet_Military"),
        (219,"ProSecurityInterview_Modifier"),
        (233,"SmugglerContacts_Modifier"),
        (239,"ImprovedTargetingVIs_Modifier"),
        (249,"CerberusCiphers_Modifier"),
        (180,"Kasumi_Military"),
        (181,"Zaeed_Military"),
        (184,"DarkEnergyDissertationUpgrade_Modifier"),
        (186,"DianaAllers_Military"),
        (187,"Normandy_Military"),
        (188,"UpgradedThanix_Modifier"),
        (199,"UpgradedHeavyShipArmor_Modifier"),
        (200,"UpgradedShield_Modifier"),
        (203,"MineralResources_Military"),
        (206,"ShialaAndZhusHopeColonists_Military"),
        (208,"Ashley_MIlitary"),
        (209,"Kaiden_MIlitary"),
        (215,"KhalisahBintSinanAlJilani_Military"),
        (266,"NeverPunchedReporter_Modifier"),
        (264,"MineralResources2_Military"),
        (265,"MineralResources3_Military"),
        (77,"AsariScienceTeam_Military"),
        (78,"Asari2ndFleet_Military"),
        (192,"AsariCommandos_Modifier"),
        (244,"ReaperCodeFragment_Modifier"),
        (79,"Asari6thFleet_Military"),
        (256,"ImprovedHuntressTraining_Modifier"),
        (80,"DestinyAscension_Military"),
        (91,"AsariCommandoTeam4_Military"),
        (92,"Samara_Military"),
        (115,"AsariScientist_Military"),
        (116,"AsariCommandoUnit1_Military"),
        (117,"AsariCommandoUnit2_Military"),
        (120,"AsariCruiser1_Military"),
        (121,"AsariResearchShips_Military"),
        (122,"AsariCruiser2_Military"),
        (123,"AsariEngineeringTeam_Military"),
        (205,"MatriarchGallaesElectronicSignature_Intel"),
        (18,"AdvancedStarshipFuel_Military"),
        (32,"ElementZeroCore_Military"),
        (54,"JavelinMissileLaunchers_Military"),
        (56,"FabricationUnits_Military"),
        (59,"VolusEngineeringTeam_Military"),
        (65,"InterfermetricArray_Military"),
        (66,"ExoGeniScientists_Military"),
        (72,"ProtheanDataFiles_Military"),
        (73,"ShadowBrokerShipTech_Military"),
        (75,"TerminusFreighters_Military"),
        (94,"EezoConverter_Military"),
        (96,"FuelPods_Military"),
        (97,"AdvancedPowerRelays_Military"),
        (99,"HaptiveOpticsArray_Military"),
        (129,"ReaperBrain_Military"),
        (130,"ReaperHeart_Military"),
        (132,"OptimizedEezoCapacitors_Military"),
        (159,"AdvancedAIRelays_Military"),
        (183,"DarkEnergyDissertation_Military"),
        (31,"CerberusResearch_Military"),
        (33,"CerberusFlotilla_Military"),
        (38,"AdvancedFighterSquadron_Military"),
        (84,"CerberusScienceTeam_Military"),
        (85,"DrBrynnCole_Military"),
        (86,"DrGavinArcher_Military"),
        (87,"Jacob_Military"),
        (133,"CerberusExPatriots_Military"),
        (136,"CerberusEscapees_External"),
        (172,"Miranda_Military"),
        (46,"Wreav_Military"),
        (47,"Wrex_Military"),
        (145,"UrdnotBetrayal_Modifier"),
        (48,"Grunt_Military"),
        (49,"AralahkCompany_Military"),
        (148,"GruntAlive_Modifier"),
        (149,"GruntLoyal_Modifier"),
        (150,"SaveQueen_Modifier"),
        (83,"Krogan1stDivision_Military"),
        (141,"KroganClans_Military"),
        (142,"ClanTurmoil_Modifier"),
        (143,"MassiveExplosion_Modifier"),
        (217,"ProKroganInterview_Modifier"),
        (258,"KaklisaurSkull_Modifier"),
        (144,"ClanUrdnot_Military"),
        (167,"MassiveExplosionA_Modifier"),
        (252,"KroganPowerGrids_Modifier"),
        (174,"KroganMercenaries_Military"),
        (102,"GethArmyCorp_Military"),
        (222,"GethInterviewGethKickAss_Modifier"),
        (103,"GethFleet_Military"),
        (221,"GethInterviewCooperation_Modifier"),
        (104,"GethPrimeC13Unit_Military"),
        (268,"DestroyTheGeth_Modifier"),
        (155,"GethFighters_Modifier"),
        (210,"GethHereticsSaved_Modifier"),
        (216,"GethHereticsDestroyed_Modifier"),
        (24,"MajorKirrahe_Military"),
        (151,"KirraheSavesSalarianCaptain_Modifier"),
        (25,"SalarianSTG1_Military"),
        (41,"Salarian1stFleet_Military"),
        (146,"MordinSolus_Military"),
        (152,"STGTaskForce_Military"),
        (171,"Salarian3rdFleet_Military"),
        (250,"SalarianColonySupport_Modifier"),
        (11,"TurianFlotilla_Military"),
        (12,"Turian6thFleet_Military"),
        (243,"CuredTurianGeneral_Modifier"),
        (42,"Turian43rdMarineDivision_Military"),
        (43,"Turian7thFleet_Military"),
        (191,"TurianMedigel_Modifier"),
        (218,"ProKroganInterviewNeedTurians_Modifier"),
        (253,"Bannerofthe1stRegimentBonus_Modifier"),
        (44,"TurianBlackwatch1_Military"),
        (45,"TurianEngineeringCorp_Military"),
        (100,"TurianSpecOpsTeam_Military"),
        (82,"AdmiralDaroXen_Military"),
        (158,"KillXen_Modifier"),
        (175,"KeepXenAlive_Modifier"),
        (101,"AdmiralZaelKoris_Military"),
        (269,"DestroyTheQuarians_Modifier"),
        (105,"QuarianCivilianFleet_Military"),
        (154,"AdmiralDies_Modifier"),
        (223,"QuarianInterviewReadiness_Modifier"),
        (106,"QuarianHeavyFleet_Military"),
        (156,"SupportRaanAgainstHanJorel_Modifier"),
        (157,"SupportHanJorelAgainstRaan_Modifier"),
        (224,"QuarianInterviewMilitary_Modifier"),
        (107,"QuarianPatrolFleet_Military"),
        (260,"ProtheanSphereBonus_Modifier"),
        (10,"BannerOfThe1stRegiment_Artifact"),
        (13,"FusionReactor_Salvage"),
        (14,"DataCache_Salvage"),
        (15,"FuelStorageDepot_Salvage"),
        (20,"PillarsOfStrength_Artifact"),
        (21,"ProcessingVIs_Salvage"),
        (22,"BattleFootage_Intel"),
        (34,"ProtheanObelisk_Artifact"),
        (35,"WeaponsCache_Salvage"),
        (36,"SecurityVI_Salvage"),
        (53,"WeaponCache_Salvage"),
        (55,"IntelligenceArchives_Intel"),
        (57,"BookOfPlenix_Artifact"),
        (64,"BattleofArcturusIntel_Intel"),
        (70,"DestroyedMiniReaper_Intel"),
        (74,"LifeSupportPods_Salvage"),
        (88,"EngineParts_Salvage"),
        (89,"LibraryOfAsha_Artifact"),
        (93,"IntactReaperGun_Intel"),
        (95,"ProtheanDataDrives_Artifact"),
        (98,"FossilizedKaklisaur_Artifact"),
        (109,"ProtheanSphere_Artifact"),
        (111,"ObeliskOfKarza_Artifact"),
        (113,"CodeOfTheAncients_Artifact"),
        (118,"EezoTanker_Salvage"),
        (119,"RingsOfAlune_Artifact"),
        (124,"HesperiaPeriodStatue_Artifact"),
        (125,"BioticResearchData_Intel"),
        (137,"GeneralSherman_External"),
        (176,"LegionIntel1_Intel"),
        (177,"LegionIntel2_Intel"),
        (178,"AdvancedBioticAmps_Intel"),
        (204,"PrejekPaddlefish_Intel"),
        (214,"FeronIntel_Intel"),
        (267,"BlackMarketArtifacts_Quest"),
    ]);

}
