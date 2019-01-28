#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved1: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved2: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved5: [u8; 200usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI,
    #[doc = "0x108 - Channel n Priority Register"]
    pub dchpri11: DCHPRI,
    #[doc = "0x109 - Channel n Priority Register"]
    pub dchpri10: DCHPRI,
    #[doc = "0x10a - Channel n Priority Register"]
    pub dchpri9: DCHPRI,
    #[doc = "0x10b - Channel n Priority Register"]
    pub dchpri8: DCHPRI,
    #[doc = "0x10c - Channel n Priority Register"]
    pub dchpri15: DCHPRI,
    #[doc = "0x10d - Channel n Priority Register"]
    pub dchpri14: DCHPRI,
    #[doc = "0x10e - Channel n Priority Register"]
    pub dchpri13: DCHPRI,
    #[doc = "0x10f - Channel n Priority Register"]
    pub dchpri12: DCHPRI,
    _reserved6: [u8; 3824usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd0_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd0_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd0_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd1_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd1_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd1_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd2_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd2_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd2_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd3_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd3_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd3_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd4_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd4_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD_CSR,
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd4_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd5_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd5_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD_CSR,
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd5_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd6_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd6_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD_CSR,
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd6_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd7_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd7_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD_CSR,
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd7_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD_ATTR,
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd8_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD_DOFF,
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd8_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD_CSR,
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd8_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD_ATTR,
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd9_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD_DOFF,
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd9_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD_CSR,
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd9_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD_ATTR,
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd10_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD_DOFF,
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd10_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD_CSR,
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd10_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD_ATTR,
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd11_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD_DOFF,
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd11_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD_CSR,
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd11_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD_ATTR,
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd12_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD_DOFF,
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd12_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD_CSR,
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd12_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD_ATTR,
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd13_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD_DOFF,
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd13_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD_CSR,
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd13_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD_ATTR,
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd14_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD_DOFF,
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd14_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD_CSR,
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd14_biter_elinkyes: TCD_BITER_ELINKYES,
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD_ATTR,
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    pub tcd15_nbytes_mloffyes: TCD_NBYTES_MLOFFYES,
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD_DOFF,
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd15_citer_elinkyes: TCD_CITER_ELINKYES,
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD_CSR,
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    pub tcd15_biter_elinkyes: TCD_BITER_ELINKYES,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register"]
pub struct ERQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register"]
pub struct EEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register"]
pub struct CEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register"]
pub struct SEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register"]
pub struct CERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register"]
pub struct SERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register"]
pub struct CDNE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register"]
pub struct SSRT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register"]
pub struct CERR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register"]
pub struct CINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD Source Address"]
pub struct TCD_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
pub struct TCD_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
pub struct TCD_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)"]
pub struct TCD_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD Destination Address"]
pub struct TCD_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
