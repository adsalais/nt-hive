// Copyright 2019-2025 Colin Finck <colin@reactos.org>
// SPDX-License-Identifier: GPL-2.0-or-later

use core::char;
use core::cmp::Ordering;
use core::fmt;

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Sorted table of lowercase Basic Multilingual Plane (BMP) character code points and their uppercase equivalents.
/// This is what Windows registry hives use to perform case-insensitive comparisons.
///
/// Generated by a modified version of
/// https://github.com/rust-lang/rust/tree/0d97f7a96877a96015d70ece41ad08bb7af12377/src/tools/unicode-table-generator
///
/// In contrast to the original UPPERCASE_TABLE in core::unicode and char::to_uppercase, this table
/// - only considers one-to-one mappings from UnicodeData.txt, but none of SpecialCasing.txt
/// - only encompasses the Unicode Basic Multilingual Plane (BMP, first 0xffff characters)
/// - uses `u16` instead of `char` for space efficiency
static BMP_UPPERCASE_TABLE: &[(u16, u16)] = &[
    (0x61, 0x41),
    (0x62, 0x42),
    (0x63, 0x43),
    (0x64, 0x44),
    (0x65, 0x45),
    (0x66, 0x46),
    (0x67, 0x47),
    (0x68, 0x48),
    (0x69, 0x49),
    (0x6a, 0x4a),
    (0x6b, 0x4b),
    (0x6c, 0x4c),
    (0x6d, 0x4d),
    (0x6e, 0x4e),
    (0x6f, 0x4f),
    (0x70, 0x50),
    (0x71, 0x51),
    (0x72, 0x52),
    (0x73, 0x53),
    (0x74, 0x54),
    (0x75, 0x55),
    (0x76, 0x56),
    (0x77, 0x57),
    (0x78, 0x58),
    (0x79, 0x59),
    (0x7a, 0x5a),
    (0xb5, 0x39c),
    (0xe0, 0xc0),
    (0xe1, 0xc1),
    (0xe2, 0xc2),
    (0xe3, 0xc3),
    (0xe4, 0xc4),
    (0xe5, 0xc5),
    (0xe6, 0xc6),
    (0xe7, 0xc7),
    (0xe8, 0xc8),
    (0xe9, 0xc9),
    (0xea, 0xca),
    (0xeb, 0xcb),
    (0xec, 0xcc),
    (0xed, 0xcd),
    (0xee, 0xce),
    (0xef, 0xcf),
    (0xf0, 0xd0),
    (0xf1, 0xd1),
    (0xf2, 0xd2),
    (0xf3, 0xd3),
    (0xf4, 0xd4),
    (0xf5, 0xd5),
    (0xf6, 0xd6),
    (0xf8, 0xd8),
    (0xf9, 0xd9),
    (0xfa, 0xda),
    (0xfb, 0xdb),
    (0xfc, 0xdc),
    (0xfd, 0xdd),
    (0xfe, 0xde),
    (0xff, 0x178),
    (0x101, 0x100),
    (0x103, 0x102),
    (0x105, 0x104),
    (0x107, 0x106),
    (0x109, 0x108),
    (0x10b, 0x10a),
    (0x10d, 0x10c),
    (0x10f, 0x10e),
    (0x111, 0x110),
    (0x113, 0x112),
    (0x115, 0x114),
    (0x117, 0x116),
    (0x119, 0x118),
    (0x11b, 0x11a),
    (0x11d, 0x11c),
    (0x11f, 0x11e),
    (0x121, 0x120),
    (0x123, 0x122),
    (0x125, 0x124),
    (0x127, 0x126),
    (0x129, 0x128),
    (0x12b, 0x12a),
    (0x12d, 0x12c),
    (0x12f, 0x12e),
    (0x131, 0x49),
    (0x133, 0x132),
    (0x135, 0x134),
    (0x137, 0x136),
    (0x13a, 0x139),
    (0x13c, 0x13b),
    (0x13e, 0x13d),
    (0x140, 0x13f),
    (0x142, 0x141),
    (0x144, 0x143),
    (0x146, 0x145),
    (0x148, 0x147),
    (0x14b, 0x14a),
    (0x14d, 0x14c),
    (0x14f, 0x14e),
    (0x151, 0x150),
    (0x153, 0x152),
    (0x155, 0x154),
    (0x157, 0x156),
    (0x159, 0x158),
    (0x15b, 0x15a),
    (0x15d, 0x15c),
    (0x15f, 0x15e),
    (0x161, 0x160),
    (0x163, 0x162),
    (0x165, 0x164),
    (0x167, 0x166),
    (0x169, 0x168),
    (0x16b, 0x16a),
    (0x16d, 0x16c),
    (0x16f, 0x16e),
    (0x171, 0x170),
    (0x173, 0x172),
    (0x175, 0x174),
    (0x177, 0x176),
    (0x17a, 0x179),
    (0x17c, 0x17b),
    (0x17e, 0x17d),
    (0x17f, 0x53),
    (0x180, 0x243),
    (0x183, 0x182),
    (0x185, 0x184),
    (0x188, 0x187),
    (0x18c, 0x18b),
    (0x192, 0x191),
    (0x195, 0x1f6),
    (0x199, 0x198),
    (0x19a, 0x23d),
    (0x19e, 0x220),
    (0x1a1, 0x1a0),
    (0x1a3, 0x1a2),
    (0x1a5, 0x1a4),
    (0x1a8, 0x1a7),
    (0x1ad, 0x1ac),
    (0x1b0, 0x1af),
    (0x1b4, 0x1b3),
    (0x1b6, 0x1b5),
    (0x1b9, 0x1b8),
    (0x1bd, 0x1bc),
    (0x1bf, 0x1f7),
    (0x1c5, 0x1c4),
    (0x1c6, 0x1c4),
    (0x1c8, 0x1c7),
    (0x1c9, 0x1c7),
    (0x1cb, 0x1ca),
    (0x1cc, 0x1ca),
    (0x1ce, 0x1cd),
    (0x1d0, 0x1cf),
    (0x1d2, 0x1d1),
    (0x1d4, 0x1d3),
    (0x1d6, 0x1d5),
    (0x1d8, 0x1d7),
    (0x1da, 0x1d9),
    (0x1dc, 0x1db),
    (0x1dd, 0x18e),
    (0x1df, 0x1de),
    (0x1e1, 0x1e0),
    (0x1e3, 0x1e2),
    (0x1e5, 0x1e4),
    (0x1e7, 0x1e6),
    (0x1e9, 0x1e8),
    (0x1eb, 0x1ea),
    (0x1ed, 0x1ec),
    (0x1ef, 0x1ee),
    (0x1f2, 0x1f1),
    (0x1f3, 0x1f1),
    (0x1f5, 0x1f4),
    (0x1f9, 0x1f8),
    (0x1fb, 0x1fa),
    (0x1fd, 0x1fc),
    (0x1ff, 0x1fe),
    (0x201, 0x200),
    (0x203, 0x202),
    (0x205, 0x204),
    (0x207, 0x206),
    (0x209, 0x208),
    (0x20b, 0x20a),
    (0x20d, 0x20c),
    (0x20f, 0x20e),
    (0x211, 0x210),
    (0x213, 0x212),
    (0x215, 0x214),
    (0x217, 0x216),
    (0x219, 0x218),
    (0x21b, 0x21a),
    (0x21d, 0x21c),
    (0x21f, 0x21e),
    (0x223, 0x222),
    (0x225, 0x224),
    (0x227, 0x226),
    (0x229, 0x228),
    (0x22b, 0x22a),
    (0x22d, 0x22c),
    (0x22f, 0x22e),
    (0x231, 0x230),
    (0x233, 0x232),
    (0x23c, 0x23b),
    (0x23f, 0x2c7e),
    (0x240, 0x2c7f),
    (0x242, 0x241),
    (0x247, 0x246),
    (0x249, 0x248),
    (0x24b, 0x24a),
    (0x24d, 0x24c),
    (0x24f, 0x24e),
    (0x250, 0x2c6f),
    (0x251, 0x2c6d),
    (0x252, 0x2c70),
    (0x253, 0x181),
    (0x254, 0x186),
    (0x256, 0x189),
    (0x257, 0x18a),
    (0x259, 0x18f),
    (0x25b, 0x190),
    (0x25c, 0xa7ab),
    (0x260, 0x193),
    (0x261, 0xa7ac),
    (0x263, 0x194),
    (0x265, 0xa78d),
    (0x266, 0xa7aa),
    (0x268, 0x197),
    (0x269, 0x196),
    (0x26a, 0xa7ae),
    (0x26b, 0x2c62),
    (0x26c, 0xa7ad),
    (0x26f, 0x19c),
    (0x271, 0x2c6e),
    (0x272, 0x19d),
    (0x275, 0x19f),
    (0x27d, 0x2c64),
    (0x280, 0x1a6),
    (0x282, 0xa7c5),
    (0x283, 0x1a9),
    (0x287, 0xa7b1),
    (0x288, 0x1ae),
    (0x289, 0x244),
    (0x28a, 0x1b1),
    (0x28b, 0x1b2),
    (0x28c, 0x245),
    (0x292, 0x1b7),
    (0x29d, 0xa7b2),
    (0x29e, 0xa7b0),
    (0x345, 0x399),
    (0x371, 0x370),
    (0x373, 0x372),
    (0x377, 0x376),
    (0x37b, 0x3fd),
    (0x37c, 0x3fe),
    (0x37d, 0x3ff),
    (0x3ac, 0x386),
    (0x3ad, 0x388),
    (0x3ae, 0x389),
    (0x3af, 0x38a),
    (0x3b1, 0x391),
    (0x3b2, 0x392),
    (0x3b3, 0x393),
    (0x3b4, 0x394),
    (0x3b5, 0x395),
    (0x3b6, 0x396),
    (0x3b7, 0x397),
    (0x3b8, 0x398),
    (0x3b9, 0x399),
    (0x3ba, 0x39a),
    (0x3bb, 0x39b),
    (0x3bc, 0x39c),
    (0x3bd, 0x39d),
    (0x3be, 0x39e),
    (0x3bf, 0x39f),
    (0x3c0, 0x3a0),
    (0x3c1, 0x3a1),
    (0x3c2, 0x3a3),
    (0x3c3, 0x3a3),
    (0x3c4, 0x3a4),
    (0x3c5, 0x3a5),
    (0x3c6, 0x3a6),
    (0x3c7, 0x3a7),
    (0x3c8, 0x3a8),
    (0x3c9, 0x3a9),
    (0x3ca, 0x3aa),
    (0x3cb, 0x3ab),
    (0x3cc, 0x38c),
    (0x3cd, 0x38e),
    (0x3ce, 0x38f),
    (0x3d0, 0x392),
    (0x3d1, 0x398),
    (0x3d5, 0x3a6),
    (0x3d6, 0x3a0),
    (0x3d7, 0x3cf),
    (0x3d9, 0x3d8),
    (0x3db, 0x3da),
    (0x3dd, 0x3dc),
    (0x3df, 0x3de),
    (0x3e1, 0x3e0),
    (0x3e3, 0x3e2),
    (0x3e5, 0x3e4),
    (0x3e7, 0x3e6),
    (0x3e9, 0x3e8),
    (0x3eb, 0x3ea),
    (0x3ed, 0x3ec),
    (0x3ef, 0x3ee),
    (0x3f0, 0x39a),
    (0x3f1, 0x3a1),
    (0x3f2, 0x3f9),
    (0x3f3, 0x37f),
    (0x3f5, 0x395),
    (0x3f8, 0x3f7),
    (0x3fb, 0x3fa),
    (0x430, 0x410),
    (0x431, 0x411),
    (0x432, 0x412),
    (0x433, 0x413),
    (0x434, 0x414),
    (0x435, 0x415),
    (0x436, 0x416),
    (0x437, 0x417),
    (0x438, 0x418),
    (0x439, 0x419),
    (0x43a, 0x41a),
    (0x43b, 0x41b),
    (0x43c, 0x41c),
    (0x43d, 0x41d),
    (0x43e, 0x41e),
    (0x43f, 0x41f),
    (0x440, 0x420),
    (0x441, 0x421),
    (0x442, 0x422),
    (0x443, 0x423),
    (0x444, 0x424),
    (0x445, 0x425),
    (0x446, 0x426),
    (0x447, 0x427),
    (0x448, 0x428),
    (0x449, 0x429),
    (0x44a, 0x42a),
    (0x44b, 0x42b),
    (0x44c, 0x42c),
    (0x44d, 0x42d),
    (0x44e, 0x42e),
    (0x44f, 0x42f),
    (0x450, 0x400),
    (0x451, 0x401),
    (0x452, 0x402),
    (0x453, 0x403),
    (0x454, 0x404),
    (0x455, 0x405),
    (0x456, 0x406),
    (0x457, 0x407),
    (0x458, 0x408),
    (0x459, 0x409),
    (0x45a, 0x40a),
    (0x45b, 0x40b),
    (0x45c, 0x40c),
    (0x45d, 0x40d),
    (0x45e, 0x40e),
    (0x45f, 0x40f),
    (0x461, 0x460),
    (0x463, 0x462),
    (0x465, 0x464),
    (0x467, 0x466),
    (0x469, 0x468),
    (0x46b, 0x46a),
    (0x46d, 0x46c),
    (0x46f, 0x46e),
    (0x471, 0x470),
    (0x473, 0x472),
    (0x475, 0x474),
    (0x477, 0x476),
    (0x479, 0x478),
    (0x47b, 0x47a),
    (0x47d, 0x47c),
    (0x47f, 0x47e),
    (0x481, 0x480),
    (0x48b, 0x48a),
    (0x48d, 0x48c),
    (0x48f, 0x48e),
    (0x491, 0x490),
    (0x493, 0x492),
    (0x495, 0x494),
    (0x497, 0x496),
    (0x499, 0x498),
    (0x49b, 0x49a),
    (0x49d, 0x49c),
    (0x49f, 0x49e),
    (0x4a1, 0x4a0),
    (0x4a3, 0x4a2),
    (0x4a5, 0x4a4),
    (0x4a7, 0x4a6),
    (0x4a9, 0x4a8),
    (0x4ab, 0x4aa),
    (0x4ad, 0x4ac),
    (0x4af, 0x4ae),
    (0x4b1, 0x4b0),
    (0x4b3, 0x4b2),
    (0x4b5, 0x4b4),
    (0x4b7, 0x4b6),
    (0x4b9, 0x4b8),
    (0x4bb, 0x4ba),
    (0x4bd, 0x4bc),
    (0x4bf, 0x4be),
    (0x4c2, 0x4c1),
    (0x4c4, 0x4c3),
    (0x4c6, 0x4c5),
    (0x4c8, 0x4c7),
    (0x4ca, 0x4c9),
    (0x4cc, 0x4cb),
    (0x4ce, 0x4cd),
    (0x4cf, 0x4c0),
    (0x4d1, 0x4d0),
    (0x4d3, 0x4d2),
    (0x4d5, 0x4d4),
    (0x4d7, 0x4d6),
    (0x4d9, 0x4d8),
    (0x4db, 0x4da),
    (0x4dd, 0x4dc),
    (0x4df, 0x4de),
    (0x4e1, 0x4e0),
    (0x4e3, 0x4e2),
    (0x4e5, 0x4e4),
    (0x4e7, 0x4e6),
    (0x4e9, 0x4e8),
    (0x4eb, 0x4ea),
    (0x4ed, 0x4ec),
    (0x4ef, 0x4ee),
    (0x4f1, 0x4f0),
    (0x4f3, 0x4f2),
    (0x4f5, 0x4f4),
    (0x4f7, 0x4f6),
    (0x4f9, 0x4f8),
    (0x4fb, 0x4fa),
    (0x4fd, 0x4fc),
    (0x4ff, 0x4fe),
    (0x501, 0x500),
    (0x503, 0x502),
    (0x505, 0x504),
    (0x507, 0x506),
    (0x509, 0x508),
    (0x50b, 0x50a),
    (0x50d, 0x50c),
    (0x50f, 0x50e),
    (0x511, 0x510),
    (0x513, 0x512),
    (0x515, 0x514),
    (0x517, 0x516),
    (0x519, 0x518),
    (0x51b, 0x51a),
    (0x51d, 0x51c),
    (0x51f, 0x51e),
    (0x521, 0x520),
    (0x523, 0x522),
    (0x525, 0x524),
    (0x527, 0x526),
    (0x529, 0x528),
    (0x52b, 0x52a),
    (0x52d, 0x52c),
    (0x52f, 0x52e),
    (0x561, 0x531),
    (0x562, 0x532),
    (0x563, 0x533),
    (0x564, 0x534),
    (0x565, 0x535),
    (0x566, 0x536),
    (0x567, 0x537),
    (0x568, 0x538),
    (0x569, 0x539),
    (0x56a, 0x53a),
    (0x56b, 0x53b),
    (0x56c, 0x53c),
    (0x56d, 0x53d),
    (0x56e, 0x53e),
    (0x56f, 0x53f),
    (0x570, 0x540),
    (0x571, 0x541),
    (0x572, 0x542),
    (0x573, 0x543),
    (0x574, 0x544),
    (0x575, 0x545),
    (0x576, 0x546),
    (0x577, 0x547),
    (0x578, 0x548),
    (0x579, 0x549),
    (0x57a, 0x54a),
    (0x57b, 0x54b),
    (0x57c, 0x54c),
    (0x57d, 0x54d),
    (0x57e, 0x54e),
    (0x57f, 0x54f),
    (0x580, 0x550),
    (0x581, 0x551),
    (0x582, 0x552),
    (0x583, 0x553),
    (0x584, 0x554),
    (0x585, 0x555),
    (0x586, 0x556),
    (0x10d0, 0x1c90),
    (0x10d1, 0x1c91),
    (0x10d2, 0x1c92),
    (0x10d3, 0x1c93),
    (0x10d4, 0x1c94),
    (0x10d5, 0x1c95),
    (0x10d6, 0x1c96),
    (0x10d7, 0x1c97),
    (0x10d8, 0x1c98),
    (0x10d9, 0x1c99),
    (0x10da, 0x1c9a),
    (0x10db, 0x1c9b),
    (0x10dc, 0x1c9c),
    (0x10dd, 0x1c9d),
    (0x10de, 0x1c9e),
    (0x10df, 0x1c9f),
    (0x10e0, 0x1ca0),
    (0x10e1, 0x1ca1),
    (0x10e2, 0x1ca2),
    (0x10e3, 0x1ca3),
    (0x10e4, 0x1ca4),
    (0x10e5, 0x1ca5),
    (0x10e6, 0x1ca6),
    (0x10e7, 0x1ca7),
    (0x10e8, 0x1ca8),
    (0x10e9, 0x1ca9),
    (0x10ea, 0x1caa),
    (0x10eb, 0x1cab),
    (0x10ec, 0x1cac),
    (0x10ed, 0x1cad),
    (0x10ee, 0x1cae),
    (0x10ef, 0x1caf),
    (0x10f0, 0x1cb0),
    (0x10f1, 0x1cb1),
    (0x10f2, 0x1cb2),
    (0x10f3, 0x1cb3),
    (0x10f4, 0x1cb4),
    (0x10f5, 0x1cb5),
    (0x10f6, 0x1cb6),
    (0x10f7, 0x1cb7),
    (0x10f8, 0x1cb8),
    (0x10f9, 0x1cb9),
    (0x10fa, 0x1cba),
    (0x10fd, 0x1cbd),
    (0x10fe, 0x1cbe),
    (0x10ff, 0x1cbf),
    (0x13f8, 0x13f0),
    (0x13f9, 0x13f1),
    (0x13fa, 0x13f2),
    (0x13fb, 0x13f3),
    (0x13fc, 0x13f4),
    (0x13fd, 0x13f5),
    (0x1c80, 0x412),
    (0x1c81, 0x414),
    (0x1c82, 0x41e),
    (0x1c83, 0x421),
    (0x1c84, 0x422),
    (0x1c85, 0x422),
    (0x1c86, 0x42a),
    (0x1c87, 0x462),
    (0x1c88, 0xa64a),
    (0x1d79, 0xa77d),
    (0x1d7d, 0x2c63),
    (0x1d8e, 0xa7c6),
    (0x1e01, 0x1e00),
    (0x1e03, 0x1e02),
    (0x1e05, 0x1e04),
    (0x1e07, 0x1e06),
    (0x1e09, 0x1e08),
    (0x1e0b, 0x1e0a),
    (0x1e0d, 0x1e0c),
    (0x1e0f, 0x1e0e),
    (0x1e11, 0x1e10),
    (0x1e13, 0x1e12),
    (0x1e15, 0x1e14),
    (0x1e17, 0x1e16),
    (0x1e19, 0x1e18),
    (0x1e1b, 0x1e1a),
    (0x1e1d, 0x1e1c),
    (0x1e1f, 0x1e1e),
    (0x1e21, 0x1e20),
    (0x1e23, 0x1e22),
    (0x1e25, 0x1e24),
    (0x1e27, 0x1e26),
    (0x1e29, 0x1e28),
    (0x1e2b, 0x1e2a),
    (0x1e2d, 0x1e2c),
    (0x1e2f, 0x1e2e),
    (0x1e31, 0x1e30),
    (0x1e33, 0x1e32),
    (0x1e35, 0x1e34),
    (0x1e37, 0x1e36),
    (0x1e39, 0x1e38),
    (0x1e3b, 0x1e3a),
    (0x1e3d, 0x1e3c),
    (0x1e3f, 0x1e3e),
    (0x1e41, 0x1e40),
    (0x1e43, 0x1e42),
    (0x1e45, 0x1e44),
    (0x1e47, 0x1e46),
    (0x1e49, 0x1e48),
    (0x1e4b, 0x1e4a),
    (0x1e4d, 0x1e4c),
    (0x1e4f, 0x1e4e),
    (0x1e51, 0x1e50),
    (0x1e53, 0x1e52),
    (0x1e55, 0x1e54),
    (0x1e57, 0x1e56),
    (0x1e59, 0x1e58),
    (0x1e5b, 0x1e5a),
    (0x1e5d, 0x1e5c),
    (0x1e5f, 0x1e5e),
    (0x1e61, 0x1e60),
    (0x1e63, 0x1e62),
    (0x1e65, 0x1e64),
    (0x1e67, 0x1e66),
    (0x1e69, 0x1e68),
    (0x1e6b, 0x1e6a),
    (0x1e6d, 0x1e6c),
    (0x1e6f, 0x1e6e),
    (0x1e71, 0x1e70),
    (0x1e73, 0x1e72),
    (0x1e75, 0x1e74),
    (0x1e77, 0x1e76),
    (0x1e79, 0x1e78),
    (0x1e7b, 0x1e7a),
    (0x1e7d, 0x1e7c),
    (0x1e7f, 0x1e7e),
    (0x1e81, 0x1e80),
    (0x1e83, 0x1e82),
    (0x1e85, 0x1e84),
    (0x1e87, 0x1e86),
    (0x1e89, 0x1e88),
    (0x1e8b, 0x1e8a),
    (0x1e8d, 0x1e8c),
    (0x1e8f, 0x1e8e),
    (0x1e91, 0x1e90),
    (0x1e93, 0x1e92),
    (0x1e95, 0x1e94),
    (0x1e9b, 0x1e60),
    (0x1ea1, 0x1ea0),
    (0x1ea3, 0x1ea2),
    (0x1ea5, 0x1ea4),
    (0x1ea7, 0x1ea6),
    (0x1ea9, 0x1ea8),
    (0x1eab, 0x1eaa),
    (0x1ead, 0x1eac),
    (0x1eaf, 0x1eae),
    (0x1eb1, 0x1eb0),
    (0x1eb3, 0x1eb2),
    (0x1eb5, 0x1eb4),
    (0x1eb7, 0x1eb6),
    (0x1eb9, 0x1eb8),
    (0x1ebb, 0x1eba),
    (0x1ebd, 0x1ebc),
    (0x1ebf, 0x1ebe),
    (0x1ec1, 0x1ec0),
    (0x1ec3, 0x1ec2),
    (0x1ec5, 0x1ec4),
    (0x1ec7, 0x1ec6),
    (0x1ec9, 0x1ec8),
    (0x1ecb, 0x1eca),
    (0x1ecd, 0x1ecc),
    (0x1ecf, 0x1ece),
    (0x1ed1, 0x1ed0),
    (0x1ed3, 0x1ed2),
    (0x1ed5, 0x1ed4),
    (0x1ed7, 0x1ed6),
    (0x1ed9, 0x1ed8),
    (0x1edb, 0x1eda),
    (0x1edd, 0x1edc),
    (0x1edf, 0x1ede),
    (0x1ee1, 0x1ee0),
    (0x1ee3, 0x1ee2),
    (0x1ee5, 0x1ee4),
    (0x1ee7, 0x1ee6),
    (0x1ee9, 0x1ee8),
    (0x1eeb, 0x1eea),
    (0x1eed, 0x1eec),
    (0x1eef, 0x1eee),
    (0x1ef1, 0x1ef0),
    (0x1ef3, 0x1ef2),
    (0x1ef5, 0x1ef4),
    (0x1ef7, 0x1ef6),
    (0x1ef9, 0x1ef8),
    (0x1efb, 0x1efa),
    (0x1efd, 0x1efc),
    (0x1eff, 0x1efe),
    (0x1f00, 0x1f08),
    (0x1f01, 0x1f09),
    (0x1f02, 0x1f0a),
    (0x1f03, 0x1f0b),
    (0x1f04, 0x1f0c),
    (0x1f05, 0x1f0d),
    (0x1f06, 0x1f0e),
    (0x1f07, 0x1f0f),
    (0x1f10, 0x1f18),
    (0x1f11, 0x1f19),
    (0x1f12, 0x1f1a),
    (0x1f13, 0x1f1b),
    (0x1f14, 0x1f1c),
    (0x1f15, 0x1f1d),
    (0x1f20, 0x1f28),
    (0x1f21, 0x1f29),
    (0x1f22, 0x1f2a),
    (0x1f23, 0x1f2b),
    (0x1f24, 0x1f2c),
    (0x1f25, 0x1f2d),
    (0x1f26, 0x1f2e),
    (0x1f27, 0x1f2f),
    (0x1f30, 0x1f38),
    (0x1f31, 0x1f39),
    (0x1f32, 0x1f3a),
    (0x1f33, 0x1f3b),
    (0x1f34, 0x1f3c),
    (0x1f35, 0x1f3d),
    (0x1f36, 0x1f3e),
    (0x1f37, 0x1f3f),
    (0x1f40, 0x1f48),
    (0x1f41, 0x1f49),
    (0x1f42, 0x1f4a),
    (0x1f43, 0x1f4b),
    (0x1f44, 0x1f4c),
    (0x1f45, 0x1f4d),
    (0x1f51, 0x1f59),
    (0x1f53, 0x1f5b),
    (0x1f55, 0x1f5d),
    (0x1f57, 0x1f5f),
    (0x1f60, 0x1f68),
    (0x1f61, 0x1f69),
    (0x1f62, 0x1f6a),
    (0x1f63, 0x1f6b),
    (0x1f64, 0x1f6c),
    (0x1f65, 0x1f6d),
    (0x1f66, 0x1f6e),
    (0x1f67, 0x1f6f),
    (0x1f70, 0x1fba),
    (0x1f71, 0x1fbb),
    (0x1f72, 0x1fc8),
    (0x1f73, 0x1fc9),
    (0x1f74, 0x1fca),
    (0x1f75, 0x1fcb),
    (0x1f76, 0x1fda),
    (0x1f77, 0x1fdb),
    (0x1f78, 0x1ff8),
    (0x1f79, 0x1ff9),
    (0x1f7a, 0x1fea),
    (0x1f7b, 0x1feb),
    (0x1f7c, 0x1ffa),
    (0x1f7d, 0x1ffb),
    (0x1f80, 0x1f88),
    (0x1f81, 0x1f89),
    (0x1f82, 0x1f8a),
    (0x1f83, 0x1f8b),
    (0x1f84, 0x1f8c),
    (0x1f85, 0x1f8d),
    (0x1f86, 0x1f8e),
    (0x1f87, 0x1f8f),
    (0x1f90, 0x1f98),
    (0x1f91, 0x1f99),
    (0x1f92, 0x1f9a),
    (0x1f93, 0x1f9b),
    (0x1f94, 0x1f9c),
    (0x1f95, 0x1f9d),
    (0x1f96, 0x1f9e),
    (0x1f97, 0x1f9f),
    (0x1fa0, 0x1fa8),
    (0x1fa1, 0x1fa9),
    (0x1fa2, 0x1faa),
    (0x1fa3, 0x1fab),
    (0x1fa4, 0x1fac),
    (0x1fa5, 0x1fad),
    (0x1fa6, 0x1fae),
    (0x1fa7, 0x1faf),
    (0x1fb0, 0x1fb8),
    (0x1fb1, 0x1fb9),
    (0x1fb3, 0x1fbc),
    (0x1fbe, 0x399),
    (0x1fc3, 0x1fcc),
    (0x1fd0, 0x1fd8),
    (0x1fd1, 0x1fd9),
    (0x1fe0, 0x1fe8),
    (0x1fe1, 0x1fe9),
    (0x1fe5, 0x1fec),
    (0x1ff3, 0x1ffc),
    (0x214e, 0x2132),
    (0x2170, 0x2160),
    (0x2171, 0x2161),
    (0x2172, 0x2162),
    (0x2173, 0x2163),
    (0x2174, 0x2164),
    (0x2175, 0x2165),
    (0x2176, 0x2166),
    (0x2177, 0x2167),
    (0x2178, 0x2168),
    (0x2179, 0x2169),
    (0x217a, 0x216a),
    (0x217b, 0x216b),
    (0x217c, 0x216c),
    (0x217d, 0x216d),
    (0x217e, 0x216e),
    (0x217f, 0x216f),
    (0x2184, 0x2183),
    (0x24d0, 0x24b6),
    (0x24d1, 0x24b7),
    (0x24d2, 0x24b8),
    (0x24d3, 0x24b9),
    (0x24d4, 0x24ba),
    (0x24d5, 0x24bb),
    (0x24d6, 0x24bc),
    (0x24d7, 0x24bd),
    (0x24d8, 0x24be),
    (0x24d9, 0x24bf),
    (0x24da, 0x24c0),
    (0x24db, 0x24c1),
    (0x24dc, 0x24c2),
    (0x24dd, 0x24c3),
    (0x24de, 0x24c4),
    (0x24df, 0x24c5),
    (0x24e0, 0x24c6),
    (0x24e1, 0x24c7),
    (0x24e2, 0x24c8),
    (0x24e3, 0x24c9),
    (0x24e4, 0x24ca),
    (0x24e5, 0x24cb),
    (0x24e6, 0x24cc),
    (0x24e7, 0x24cd),
    (0x24e8, 0x24ce),
    (0x24e9, 0x24cf),
    (0x2c30, 0x2c00),
    (0x2c31, 0x2c01),
    (0x2c32, 0x2c02),
    (0x2c33, 0x2c03),
    (0x2c34, 0x2c04),
    (0x2c35, 0x2c05),
    (0x2c36, 0x2c06),
    (0x2c37, 0x2c07),
    (0x2c38, 0x2c08),
    (0x2c39, 0x2c09),
    (0x2c3a, 0x2c0a),
    (0x2c3b, 0x2c0b),
    (0x2c3c, 0x2c0c),
    (0x2c3d, 0x2c0d),
    (0x2c3e, 0x2c0e),
    (0x2c3f, 0x2c0f),
    (0x2c40, 0x2c10),
    (0x2c41, 0x2c11),
    (0x2c42, 0x2c12),
    (0x2c43, 0x2c13),
    (0x2c44, 0x2c14),
    (0x2c45, 0x2c15),
    (0x2c46, 0x2c16),
    (0x2c47, 0x2c17),
    (0x2c48, 0x2c18),
    (0x2c49, 0x2c19),
    (0x2c4a, 0x2c1a),
    (0x2c4b, 0x2c1b),
    (0x2c4c, 0x2c1c),
    (0x2c4d, 0x2c1d),
    (0x2c4e, 0x2c1e),
    (0x2c4f, 0x2c1f),
    (0x2c50, 0x2c20),
    (0x2c51, 0x2c21),
    (0x2c52, 0x2c22),
    (0x2c53, 0x2c23),
    (0x2c54, 0x2c24),
    (0x2c55, 0x2c25),
    (0x2c56, 0x2c26),
    (0x2c57, 0x2c27),
    (0x2c58, 0x2c28),
    (0x2c59, 0x2c29),
    (0x2c5a, 0x2c2a),
    (0x2c5b, 0x2c2b),
    (0x2c5c, 0x2c2c),
    (0x2c5d, 0x2c2d),
    (0x2c5e, 0x2c2e),
    (0x2c61, 0x2c60),
    (0x2c65, 0x23a),
    (0x2c66, 0x23e),
    (0x2c68, 0x2c67),
    (0x2c6a, 0x2c69),
    (0x2c6c, 0x2c6b),
    (0x2c73, 0x2c72),
    (0x2c76, 0x2c75),
    (0x2c81, 0x2c80),
    (0x2c83, 0x2c82),
    (0x2c85, 0x2c84),
    (0x2c87, 0x2c86),
    (0x2c89, 0x2c88),
    (0x2c8b, 0x2c8a),
    (0x2c8d, 0x2c8c),
    (0x2c8f, 0x2c8e),
    (0x2c91, 0x2c90),
    (0x2c93, 0x2c92),
    (0x2c95, 0x2c94),
    (0x2c97, 0x2c96),
    (0x2c99, 0x2c98),
    (0x2c9b, 0x2c9a),
    (0x2c9d, 0x2c9c),
    (0x2c9f, 0x2c9e),
    (0x2ca1, 0x2ca0),
    (0x2ca3, 0x2ca2),
    (0x2ca5, 0x2ca4),
    (0x2ca7, 0x2ca6),
    (0x2ca9, 0x2ca8),
    (0x2cab, 0x2caa),
    (0x2cad, 0x2cac),
    (0x2caf, 0x2cae),
    (0x2cb1, 0x2cb0),
    (0x2cb3, 0x2cb2),
    (0x2cb5, 0x2cb4),
    (0x2cb7, 0x2cb6),
    (0x2cb9, 0x2cb8),
    (0x2cbb, 0x2cba),
    (0x2cbd, 0x2cbc),
    (0x2cbf, 0x2cbe),
    (0x2cc1, 0x2cc0),
    (0x2cc3, 0x2cc2),
    (0x2cc5, 0x2cc4),
    (0x2cc7, 0x2cc6),
    (0x2cc9, 0x2cc8),
    (0x2ccb, 0x2cca),
    (0x2ccd, 0x2ccc),
    (0x2ccf, 0x2cce),
    (0x2cd1, 0x2cd0),
    (0x2cd3, 0x2cd2),
    (0x2cd5, 0x2cd4),
    (0x2cd7, 0x2cd6),
    (0x2cd9, 0x2cd8),
    (0x2cdb, 0x2cda),
    (0x2cdd, 0x2cdc),
    (0x2cdf, 0x2cde),
    (0x2ce1, 0x2ce0),
    (0x2ce3, 0x2ce2),
    (0x2cec, 0x2ceb),
    (0x2cee, 0x2ced),
    (0x2cf3, 0x2cf2),
    (0x2d00, 0x10a0),
    (0x2d01, 0x10a1),
    (0x2d02, 0x10a2),
    (0x2d03, 0x10a3),
    (0x2d04, 0x10a4),
    (0x2d05, 0x10a5),
    (0x2d06, 0x10a6),
    (0x2d07, 0x10a7),
    (0x2d08, 0x10a8),
    (0x2d09, 0x10a9),
    (0x2d0a, 0x10aa),
    (0x2d0b, 0x10ab),
    (0x2d0c, 0x10ac),
    (0x2d0d, 0x10ad),
    (0x2d0e, 0x10ae),
    (0x2d0f, 0x10af),
    (0x2d10, 0x10b0),
    (0x2d11, 0x10b1),
    (0x2d12, 0x10b2),
    (0x2d13, 0x10b3),
    (0x2d14, 0x10b4),
    (0x2d15, 0x10b5),
    (0x2d16, 0x10b6),
    (0x2d17, 0x10b7),
    (0x2d18, 0x10b8),
    (0x2d19, 0x10b9),
    (0x2d1a, 0x10ba),
    (0x2d1b, 0x10bb),
    (0x2d1c, 0x10bc),
    (0x2d1d, 0x10bd),
    (0x2d1e, 0x10be),
    (0x2d1f, 0x10bf),
    (0x2d20, 0x10c0),
    (0x2d21, 0x10c1),
    (0x2d22, 0x10c2),
    (0x2d23, 0x10c3),
    (0x2d24, 0x10c4),
    (0x2d25, 0x10c5),
    (0x2d27, 0x10c7),
    (0x2d2d, 0x10cd),
    (0xa641, 0xa640),
    (0xa643, 0xa642),
    (0xa645, 0xa644),
    (0xa647, 0xa646),
    (0xa649, 0xa648),
    (0xa64b, 0xa64a),
    (0xa64d, 0xa64c),
    (0xa64f, 0xa64e),
    (0xa651, 0xa650),
    (0xa653, 0xa652),
    (0xa655, 0xa654),
    (0xa657, 0xa656),
    (0xa659, 0xa658),
    (0xa65b, 0xa65a),
    (0xa65d, 0xa65c),
    (0xa65f, 0xa65e),
    (0xa661, 0xa660),
    (0xa663, 0xa662),
    (0xa665, 0xa664),
    (0xa667, 0xa666),
    (0xa669, 0xa668),
    (0xa66b, 0xa66a),
    (0xa66d, 0xa66c),
    (0xa681, 0xa680),
    (0xa683, 0xa682),
    (0xa685, 0xa684),
    (0xa687, 0xa686),
    (0xa689, 0xa688),
    (0xa68b, 0xa68a),
    (0xa68d, 0xa68c),
    (0xa68f, 0xa68e),
    (0xa691, 0xa690),
    (0xa693, 0xa692),
    (0xa695, 0xa694),
    (0xa697, 0xa696),
    (0xa699, 0xa698),
    (0xa69b, 0xa69a),
    (0xa723, 0xa722),
    (0xa725, 0xa724),
    (0xa727, 0xa726),
    (0xa729, 0xa728),
    (0xa72b, 0xa72a),
    (0xa72d, 0xa72c),
    (0xa72f, 0xa72e),
    (0xa733, 0xa732),
    (0xa735, 0xa734),
    (0xa737, 0xa736),
    (0xa739, 0xa738),
    (0xa73b, 0xa73a),
    (0xa73d, 0xa73c),
    (0xa73f, 0xa73e),
    (0xa741, 0xa740),
    (0xa743, 0xa742),
    (0xa745, 0xa744),
    (0xa747, 0xa746),
    (0xa749, 0xa748),
    (0xa74b, 0xa74a),
    (0xa74d, 0xa74c),
    (0xa74f, 0xa74e),
    (0xa751, 0xa750),
    (0xa753, 0xa752),
    (0xa755, 0xa754),
    (0xa757, 0xa756),
    (0xa759, 0xa758),
    (0xa75b, 0xa75a),
    (0xa75d, 0xa75c),
    (0xa75f, 0xa75e),
    (0xa761, 0xa760),
    (0xa763, 0xa762),
    (0xa765, 0xa764),
    (0xa767, 0xa766),
    (0xa769, 0xa768),
    (0xa76b, 0xa76a),
    (0xa76d, 0xa76c),
    (0xa76f, 0xa76e),
    (0xa77a, 0xa779),
    (0xa77c, 0xa77b),
    (0xa77f, 0xa77e),
    (0xa781, 0xa780),
    (0xa783, 0xa782),
    (0xa785, 0xa784),
    (0xa787, 0xa786),
    (0xa78c, 0xa78b),
    (0xa791, 0xa790),
    (0xa793, 0xa792),
    (0xa794, 0xa7c4),
    (0xa797, 0xa796),
    (0xa799, 0xa798),
    (0xa79b, 0xa79a),
    (0xa79d, 0xa79c),
    (0xa79f, 0xa79e),
    (0xa7a1, 0xa7a0),
    (0xa7a3, 0xa7a2),
    (0xa7a5, 0xa7a4),
    (0xa7a7, 0xa7a6),
    (0xa7a9, 0xa7a8),
    (0xa7b5, 0xa7b4),
    (0xa7b7, 0xa7b6),
    (0xa7b9, 0xa7b8),
    (0xa7bb, 0xa7ba),
    (0xa7bd, 0xa7bc),
    (0xa7bf, 0xa7be),
    (0xa7c3, 0xa7c2),
    (0xa7c8, 0xa7c7),
    (0xa7ca, 0xa7c9),
    (0xa7f6, 0xa7f5),
    (0xab53, 0xa7b3),
    (0xab70, 0x13a0),
    (0xab71, 0x13a1),
    (0xab72, 0x13a2),
    (0xab73, 0x13a3),
    (0xab74, 0x13a4),
    (0xab75, 0x13a5),
    (0xab76, 0x13a6),
    (0xab77, 0x13a7),
    (0xab78, 0x13a8),
    (0xab79, 0x13a9),
    (0xab7a, 0x13aa),
    (0xab7b, 0x13ab),
    (0xab7c, 0x13ac),
    (0xab7d, 0x13ad),
    (0xab7e, 0x13ae),
    (0xab7f, 0x13af),
    (0xab80, 0x13b0),
    (0xab81, 0x13b1),
    (0xab82, 0x13b2),
    (0xab83, 0x13b3),
    (0xab84, 0x13b4),
    (0xab85, 0x13b5),
    (0xab86, 0x13b6),
    (0xab87, 0x13b7),
    (0xab88, 0x13b8),
    (0xab89, 0x13b9),
    (0xab8a, 0x13ba),
    (0xab8b, 0x13bb),
    (0xab8c, 0x13bc),
    (0xab8d, 0x13bd),
    (0xab8e, 0x13be),
    (0xab8f, 0x13bf),
    (0xab90, 0x13c0),
    (0xab91, 0x13c1),
    (0xab92, 0x13c2),
    (0xab93, 0x13c3),
    (0xab94, 0x13c4),
    (0xab95, 0x13c5),
    (0xab96, 0x13c6),
    (0xab97, 0x13c7),
    (0xab98, 0x13c8),
    (0xab99, 0x13c9),
    (0xab9a, 0x13ca),
    (0xab9b, 0x13cb),
    (0xab9c, 0x13cc),
    (0xab9d, 0x13cd),
    (0xab9e, 0x13ce),
    (0xab9f, 0x13cf),
    (0xaba0, 0x13d0),
    (0xaba1, 0x13d1),
    (0xaba2, 0x13d2),
    (0xaba3, 0x13d3),
    (0xaba4, 0x13d4),
    (0xaba5, 0x13d5),
    (0xaba6, 0x13d6),
    (0xaba7, 0x13d7),
    (0xaba8, 0x13d8),
    (0xaba9, 0x13d9),
    (0xabaa, 0x13da),
    (0xabab, 0x13db),
    (0xabac, 0x13dc),
    (0xabad, 0x13dd),
    (0xabae, 0x13de),
    (0xabaf, 0x13df),
    (0xabb0, 0x13e0),
    (0xabb1, 0x13e1),
    (0xabb2, 0x13e2),
    (0xabb3, 0x13e3),
    (0xabb4, 0x13e4),
    (0xabb5, 0x13e5),
    (0xabb6, 0x13e6),
    (0xabb7, 0x13e7),
    (0xabb8, 0x13e8),
    (0xabb9, 0x13e9),
    (0xabba, 0x13ea),
    (0xabbb, 0x13eb),
    (0xabbc, 0x13ec),
    (0xabbd, 0x13ed),
    (0xabbe, 0x13ee),
    (0xabbf, 0x13ef),
    (0xff41, 0xff21),
    (0xff42, 0xff22),
    (0xff43, 0xff23),
    (0xff44, 0xff24),
    (0xff45, 0xff25),
    (0xff46, 0xff26),
    (0xff47, 0xff27),
    (0xff48, 0xff28),
    (0xff49, 0xff29),
    (0xff4a, 0xff2a),
    (0xff4b, 0xff2b),
    (0xff4c, 0xff2c),
    (0xff4d, 0xff2d),
    (0xff4e, 0xff2e),
    (0xff4f, 0xff2f),
    (0xff50, 0xff30),
    (0xff51, 0xff31),
    (0xff52, 0xff32),
    (0xff53, 0xff33),
    (0xff54, 0xff34),
    (0xff55, 0xff35),
    (0xff56, 0xff36),
    (0xff57, 0xff37),
    (0xff58, 0xff38),
    (0xff59, 0xff39),
    (0xff5a, 0xff3a),
];

fn utf16_code_unit_to_uppercase(unit: u16) -> u16 {
    match BMP_UPPERCASE_TABLE.binary_search_by(|&(key, _)| key.cmp(&unit)) {
        Ok(index) => BMP_UPPERCASE_TABLE[index].1,
        Err(_) => unit,
    }
}

/// Zero-copy representation of a key name or value name string stored in hive data.
/// Can be either in Latin1 (ISO-8859-1) or UTF-16 (Little-Endian).
///
/// This allows to work with the string without performing any allocations or conversions.
/// If the `alloc` feature is enabled, [`to_string_checked`](NtHiveNameString::to_string_checked) and
/// [`to_string_lossy`](NtHiveNameString::to_string_lossy) can be used to to retrieve a `String`.
#[derive(Clone, Debug, Eq)]
pub enum NtHiveNameString<'h> {
    /// A byte stream where each byte is a single character of the Latin1 (ISO-8859-1)
    /// character set.
    /// Each byte can simply be casted to a [`prim@char`] (as Unicode is ordered the same as Latin1).
    Latin1(&'h [u8]),
    /// A byte stream where every two bytes make up a UTF-16 code point in little-endian order.
    /// Use [`u16::from_le_bytes`] and [`char::decode_utf16`] if you want to get a stream of [`prim@char`]s.
    Utf16LE(&'h [u8]),
}

impl<'h> NtHiveNameString<'h> {
    fn cmp_iter<TI, OI>(mut this_iter: TI, mut other_iter: OI) -> Ordering
    where
        TI: Iterator<Item = u16>,
        OI: Iterator<Item = u16>,
    {
        loop {
            match (this_iter.next(), other_iter.next()) {
                (Some(this_code_unit), Some(other_code_unit)) => {
                    // We have two UTF-16 code units to compare.
                    let this_upper = utf16_code_unit_to_uppercase(this_code_unit);
                    let other_upper = utf16_code_unit_to_uppercase(other_code_unit);

                    if this_upper != other_upper {
                        return this_upper.cmp(&other_upper);
                    }
                }
                (Some(_), None) => {
                    // `this_iter` is longer than `other_iter` but otherwise equal.
                    return Ordering::Greater;
                }
                (None, Some(_)) => {
                    // `other_iter` is longer than `this_iter` but otherwise equal.
                    return Ordering::Less;
                }
                (None, None) => {
                    // We made it to the end of both strings, so they must be equal.
                    return Ordering::Equal;
                }
            }
        }
    }

    fn cmp_self_and_str(lhs: &Self, rhs: &str) -> Ordering {
        let rhs_iter = rhs.encode_utf16();

        match lhs {
            Self::Latin1(_) => Self::cmp_iter(lhs.latin1_iter(), rhs_iter),
            Self::Utf16LE(_) => Self::cmp_iter(lhs.utf16le_iter(), rhs_iter),
        }
    }

    fn cmp_str_and_self(lhs: &str, rhs: &Self) -> Ordering {
        let lhs_iter = lhs.encode_utf16();

        match rhs {
            Self::Latin1(_) => Self::cmp_iter(lhs_iter, rhs.latin1_iter()),
            Self::Utf16LE(_) => Self::cmp_iter(lhs_iter, rhs.utf16le_iter()),
        }
    }

    fn latin1_iter(&'h self) -> impl Iterator<Item = u16> + 'h {
        match self {
            Self::Latin1(bytes) => bytes.iter().map(|byte| *byte as u16),
            Self::Utf16LE(_) => panic!("Called latin1_iter for Utf16LE"),
        }
    }

    fn utf16le_iter(&'h self) -> impl Iterator<Item = u16> + 'h {
        match self {
            Self::Latin1(_) => panic!("Called utf16le_iter for Latin1"),
            Self::Utf16LE(bytes) => bytes
                .chunks_exact(2)
                .map(|two_bytes| u16::from_le_bytes(two_bytes.try_into().unwrap())),
        }
    }

    /// Returns `true` if `self` has a length of zero bytes.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of `self`.
    ///
    /// This length is in bytes, not characters! In other words,
    /// it may not be what a human considers the length of the string.
    pub const fn len(&self) -> usize {
        match self {
            Self::Latin1(bytes) => bytes.len(),
            Self::Utf16LE(bytes) => bytes.len(),
        }
    }

    /// Attempts to convert `self` to an owned `String`.
    /// Returns `Some(String)` if all characters could be converted successfully or `None` if a decoding error occurred.
    #[cfg(feature = "alloc")]
    pub fn to_string_checked(&self) -> Option<String> {
        match self {
            Self::Latin1(bytes) => {
                let string = bytes.iter().map(|byte| *byte as char).collect();
                Some(string)
            }
            Self::Utf16LE(_) => char::decode_utf16(self.utf16le_iter())
                .map(|x| x.ok())
                .collect::<Option<String>>(),
        }
    }

    /// Converts `self` to an owned `String`, replacing invalid data with the replacement character (U+FFFD).
    #[cfg(feature = "alloc")]
    pub fn to_string_lossy(&self) -> String {
        match self {
            Self::Latin1(bytes) => bytes.iter().map(|byte| *byte as char).collect(),
            Self::Utf16LE(_) => char::decode_utf16(self.utf16le_iter())
                .map(|x| x.unwrap_or(char::REPLACEMENT_CHARACTER))
                .collect(),
        }
    }
}

impl fmt::Display for NtHiveNameString<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Latin1(bytes) => {
                for byte in bytes.iter() {
                    let single_char = *byte as char;
                    single_char.fmt(f)?;
                }
            }
            Self::Utf16LE(_) => {
                let utf16_iter = char::decode_utf16(self.utf16le_iter())
                    .map(|x| x.unwrap_or(char::REPLACEMENT_CHARACTER));

                for single_char in utf16_iter {
                    single_char.fmt(f)?;
                }
            }
        }

        Ok(())
    }
}

impl Ord for NtHiveNameString<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Latin1(_), Self::Latin1(_)) => {
                Self::cmp_iter(self.latin1_iter(), other.latin1_iter())
            }
            (Self::Latin1(_), Self::Utf16LE(_)) => {
                Self::cmp_iter(self.latin1_iter(), other.utf16le_iter())
            }
            (Self::Utf16LE(_), Self::Latin1(_)) => {
                Self::cmp_iter(self.utf16le_iter(), other.latin1_iter())
            }
            (Self::Utf16LE(_), Self::Utf16LE(_)) => {
                Self::cmp_iter(self.utf16le_iter(), other.utf16le_iter())
            }
        }
    }
}

impl PartialEq for NtHiveNameString<'_> {
    /// Checks that two strings are a case-insensitive match
    /// (according to Windows' definition of case-insensitivity, which only considers the
    /// Unicode Basic Multilingual Plane).
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialEq<str> for NtHiveNameString<'_> {
    fn eq(&self, other: &str) -> bool {
        NtHiveNameString::cmp_self_and_str(self, other) == Ordering::Equal
    }
}

impl<'h> PartialEq<NtHiveNameString<'h>> for str {
    fn eq(&self, other: &NtHiveNameString<'h>) -> bool {
        NtHiveNameString::cmp_str_and_self(self, other) == Ordering::Equal
    }
}

impl PartialEq<&str> for NtHiveNameString<'_> {
    fn eq(&self, other: &&str) -> bool {
        NtHiveNameString::cmp_self_and_str(self, other) == Ordering::Equal
    }
}

impl<'h> PartialEq<NtHiveNameString<'h>> for &str {
    fn eq(&self, other: &NtHiveNameString<'h>) -> bool {
        NtHiveNameString::cmp_str_and_self(self, other) == Ordering::Equal
    }
}

impl PartialOrd for NtHiveNameString<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for NtHiveNameString<'_> {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(NtHiveNameString::cmp_self_and_str(self, other))
    }
}

impl<'h> PartialOrd<NtHiveNameString<'h>> for str {
    fn partial_cmp(&self, other: &NtHiveNameString<'h>) -> Option<Ordering> {
        Some(NtHiveNameString::cmp_str_and_self(self, other))
    }
}

impl PartialOrd<&str> for NtHiveNameString<'_> {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        Some(NtHiveNameString::cmp_self_and_str(self, other))
    }
}

impl<'h> PartialOrd<NtHiveNameString<'h>> for &str {
    fn partial_cmp(&self, other: &NtHiveNameString<'h>) -> Option<Ordering> {
        Some(NtHiveNameString::cmp_str_and_self(self, other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        assert_eq!(NtHiveNameString::Latin1(b"Hello"), "Hello");
        assert_eq!(
            NtHiveNameString::Utf16LE(&[b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]),
            "Hello"
        );
        assert_eq!(NtHiveNameString::Latin1(b"Hello"), "hello");
        assert_eq!(
            NtHiveNameString::Utf16LE(&[b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]),
            "hello"
        );
        assert_eq!(NtHiveNameString::Latin1(b"Hell\xD6"), "hellö");
        assert_ne!(NtHiveNameString::Latin1(b"Hello"), "Hell");
        assert_ne!(
            NtHiveNameString::Utf16LE(&[b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]),
            "Hell"
        );

        // Characters in the Basic Multilingual Plane are compared case-insensitively,
        // others are not.
        let full_width_upper_a = "\u{FF21}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        let full_width_lower_a = "\u{FF41}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        assert_eq!(
            NtHiveNameString::Utf16LE(&full_width_upper_a),
            NtHiveNameString::Utf16LE(&full_width_lower_a)
        );

        let deseret_upper_h = "\u{10410}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        let deseret_lower_h = "\u{10438}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        assert_ne!(
            NtHiveNameString::Utf16LE(&deseret_upper_h),
            NtHiveNameString::Utf16LE(&deseret_lower_h)
        );
    }

    #[test]
    fn test_is_empty() {
        assert!(NtHiveNameString::Latin1(b"").is_empty());
        assert!(NtHiveNameString::Utf16LE(&[]).is_empty());
        assert!(!NtHiveNameString::Latin1(b"Hello").is_empty());
        assert!(
            !NtHiveNameString::Utf16LE(&[b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]).is_empty()
        );
    }

    #[test]
    fn test_len() {
        assert_eq!(NtHiveNameString::Latin1(b"Hello").len(), 5);
        assert_eq!(
            NtHiveNameString::Utf16LE(&[b'H', 0, b'e', 0, b'l', 0, b'l', 0, b'o', 0]).len(),
            10
        );
    }

    #[test]
    fn test_ord() {
        assert!(NtHiveNameString::Latin1(b"a") < "b");
        assert!("b" > NtHiveNameString::Latin1(b"a"));

        assert!(NtHiveNameString::Latin1(b"a") == NtHiveNameString::Latin1(b"a"));
        assert!(NtHiveNameString::Latin1(b"a") < NtHiveNameString::Latin1(b"aa"));
        assert!(NtHiveNameString::Latin1(b"aa") > NtHiveNameString::Latin1(b"a"));
        assert!(NtHiveNameString::Latin1(b"a") < NtHiveNameString::Latin1(b"b"));

        // Even though Unicode character 0x10331 (Gothic Letter Bairkan) has a higher
        // code point than Unicode character 0xFF21 (Full-Width Latin Capital Letter A), Windows
        // hives order name strings by their UTF-16 representation.
        // Hence, 0x10331 is encoded as 0xD800 0xDF31 and comes before 0xFF21
        // (which is the same in UTF-16).
        let full_width_a = "\u{FF21}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        let gothic_bairkan = "\u{10331}"
            .encode_utf16()
            .flat_map(|utf16_code_point| utf16_code_point.to_le_bytes().to_vec())
            .collect::<Vec<u8>>();
        assert!(
            NtHiveNameString::Utf16LE(&gothic_bairkan) < NtHiveNameString::Utf16LE(&full_width_a)
        );
    }
}
