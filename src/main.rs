#![allow(dead_code)]
use std::{collections::HashMap, time::Instant};

fn e1() -> &'static str {
    "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
"
}
fn e2() -> &'static str {
    "\
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
"
}
fn e3() -> &'static str {
    "\
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
"
}
fn e4() -> &'static str {
    "\
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"
}
fn d() -> &'static str {
    "\
Button A: X+31, Y+16
Button B: X+35, Y+66
Prize: X=16140, Y=5352

Button A: X+16, Y+90
Button B: X+49, Y+38
Prize: X=4916, Y=7692

Button A: X+16, Y+56
Button B: X+65, Y+17
Prize: X=6021, Y=3181

Button A: X+35, Y+99
Button B: X+65, Y+15
Prize: X=3970, Y=4644

Button A: X+32, Y+83
Button B: X+65, Y+14
Prize: X=15644, Y=14165

Button A: X+46, Y+14
Button B: X+13, Y+36
Prize: X=1251, Y=19212

Button A: X+36, Y+80
Button B: X+63, Y+42
Prize: X=2520, Y=4032

Button A: X+62, Y+26
Button B: X+16, Y+44
Prize: X=10432, Y=15392

Button A: X+28, Y+38
Button B: X+37, Y+15
Prize: X=4058, Y=2338

Button A: X+57, Y+74
Button B: X+85, Y+22
Prize: X=7902, Y=6548

Button A: X+93, Y+40
Button B: X+13, Y+52
Prize: X=4521, Y=3476

Button A: X+46, Y+14
Button B: X+40, Y+94
Prize: X=7870, Y=10496

Button A: X+83, Y+11
Button B: X+13, Y+86
Prize: X=11513, Y=6031

Button A: X+37, Y+86
Button B: X+62, Y+30
Prize: X=9061, Y=9764

Button A: X+94, Y+87
Button B: X+24, Y+89
Prize: X=2174, Y=3615

Button A: X+71, Y+14
Button B: X+76, Y+93
Prize: X=8827, Y=4315

Button A: X+49, Y+14
Button B: X+14, Y+74
Prize: X=5520, Y=4340

Button A: X+11, Y+37
Button B: X+78, Y+28
Prize: X=1891, Y=11769

Button A: X+14, Y+58
Button B: X+47, Y+13
Prize: X=11773, Y=5847

Button A: X+23, Y+75
Button B: X+79, Y+13
Prize: X=5901, Y=2609

Button A: X+35, Y+89
Button B: X+70, Y+26
Prize: X=3115, Y=2601

Button A: X+47, Y+15
Button B: X+27, Y+66
Prize: X=11481, Y=11627

Button A: X+95, Y+40
Button B: X+12, Y+62
Prize: X=9854, Y=8534

Button A: X+60, Y+29
Button B: X+22, Y+66
Prize: X=2654, Y=2224

Button A: X+52, Y+84
Button B: X+62, Y+33
Prize: X=5426, Y=3057

Button A: X+15, Y+68
Button B: X+90, Y+64
Prize: X=7650, Y=9224

Button A: X+61, Y+28
Button B: X+22, Y+46
Prize: X=4513, Y=16294

Button A: X+83, Y+18
Button B: X+85, Y+95
Prize: X=8798, Y=8263

Button A: X+70, Y+11
Button B: X+13, Y+59
Prize: X=16445, Y=15476

Button A: X+49, Y+83
Button B: X+96, Y+27
Prize: X=9314, Y=9403

Button A: X+22, Y+28
Button B: X+87, Y+31
Prize: X=5484, Y=3950

Button A: X+65, Y+93
Button B: X+38, Y+11
Prize: X=5770, Y=7605

Button A: X+83, Y+18
Button B: X+73, Y+84
Prize: X=8499, Y=2184

Button A: X+76, Y+13
Button B: X+18, Y+75
Prize: X=15744, Y=17394

Button A: X+11, Y+43
Button B: X+57, Y+29
Prize: X=470, Y=7922

Button A: X+91, Y+14
Button B: X+24, Y+26
Prize: X=2014, Y=1336

Button A: X+58, Y+13
Button B: X+24, Y+75
Prize: X=6210, Y=3759

Button A: X+78, Y+13
Button B: X+34, Y+34
Prize: X=7874, Y=3324

Button A: X+39, Y+17
Button B: X+22, Y+39
Prize: X=18357, Y=16009

Button A: X+39, Y+12
Button B: X+22, Y+33
Prize: X=3779, Y=12617

Button A: X+16, Y+68
Button B: X+33, Y+11
Prize: X=2036, Y=4852

Button A: X+15, Y+49
Button B: X+76, Y+34
Prize: X=2865, Y=19593

Button A: X+67, Y+25
Button B: X+17, Y+67
Prize: X=12786, Y=17878

Button A: X+26, Y+55
Button B: X+79, Y+46
Prize: X=1339, Y=1258

Button A: X+26, Y+59
Button B: X+43, Y+22
Prize: X=1279, Y=4921

Button A: X+55, Y+37
Button B: X+18, Y+42
Prize: X=15840, Y=9912

Button A: X+14, Y+45
Button B: X+31, Y+12
Prize: X=12133, Y=8264

Button A: X+75, Y+20
Button B: X+17, Y+75
Prize: X=10737, Y=6235

Button A: X+16, Y+95
Button B: X+90, Y+35
Prize: X=8946, Y=6675

Button A: X+27, Y+12
Button B: X+33, Y+56
Prize: X=5223, Y=5752

Button A: X+14, Y+49
Button B: X+28, Y+14
Prize: X=3178, Y=3815

Button A: X+70, Y+29
Button B: X+24, Y+57
Prize: X=13804, Y=5949

Button A: X+16, Y+61
Button B: X+50, Y+29
Prize: X=2972, Y=2603

Button A: X+13, Y+32
Button B: X+53, Y+14
Prize: X=1766, Y=15426

Button A: X+59, Y+20
Button B: X+20, Y+46
Prize: X=14383, Y=5868

Button A: X+26, Y+85
Button B: X+85, Y+77
Prize: X=8351, Y=12235

Button A: X+38, Y+20
Button B: X+19, Y+50
Prize: X=4851, Y=14530

Button A: X+11, Y+71
Button B: X+58, Y+15
Prize: X=17657, Y=9515

Button A: X+43, Y+14
Button B: X+51, Y+82
Prize: X=1308, Y=19148

Button A: X+24, Y+30
Button B: X+99, Y+30
Prize: X=6882, Y=2790

Button A: X+33, Y+65
Button B: X+44, Y+14
Prize: X=2255, Y=1971

Button A: X+22, Y+56
Button B: X+34, Y+14
Prize: X=11602, Y=7452

Button A: X+36, Y+14
Button B: X+59, Y+82
Prize: X=16525, Y=18080

Button A: X+99, Y+22
Button B: X+49, Y+50
Prize: X=10127, Y=3854

Button A: X+41, Y+21
Button B: X+41, Y+67
Prize: X=3510, Y=4822

Button A: X+88, Y+65
Button B: X+27, Y+92
Prize: X=1318, Y=2847

Button A: X+88, Y+60
Button B: X+17, Y+57
Prize: X=1987, Y=1491

Button A: X+34, Y+65
Button B: X+53, Y+21
Prize: X=12096, Y=4212

Button A: X+97, Y+24
Button B: X+82, Y+97
Prize: X=6702, Y=7565

Button A: X+81, Y+49
Button B: X+22, Y+74
Prize: X=1479, Y=1623

Button A: X+34, Y+75
Button B: X+56, Y+34
Prize: X=1066, Y=919

Button A: X+43, Y+77
Button B: X+51, Y+24
Prize: X=4588, Y=2897

Button A: X+24, Y+48
Button B: X+27, Y+16
Prize: X=19385, Y=15888

Button A: X+72, Y+15
Button B: X+11, Y+38
Prize: X=12193, Y=19018

Button A: X+84, Y+40
Button B: X+41, Y+75
Prize: X=9821, Y=7395

Button A: X+68, Y+30
Button B: X+47, Y+88
Prize: X=5114, Y=3736

Button A: X+40, Y+78
Button B: X+50, Y+18
Prize: X=10020, Y=16988

Button A: X+42, Y+70
Button B: X+43, Y+18
Prize: X=12990, Y=18348

Button A: X+83, Y+21
Button B: X+25, Y+85
Prize: X=6500, Y=2510

Button A: X+20, Y+60
Button B: X+53, Y+25
Prize: X=2784, Y=7720

Button A: X+61, Y+14
Button B: X+42, Y+58
Prize: X=2974, Y=3826

Button A: X+14, Y+55
Button B: X+31, Y+31
Prize: X=2376, Y=6066

Button A: X+96, Y+62
Button B: X+14, Y+55
Prize: X=6144, Y=6174

Button A: X+99, Y+44
Button B: X+46, Y+71
Prize: X=9066, Y=4636

Button A: X+11, Y+63
Button B: X+76, Y+20
Prize: X=4293, Y=2993

Button A: X+91, Y+20
Button B: X+41, Y+45
Prize: X=10223, Y=4910

Button A: X+32, Y+14
Button B: X+11, Y+39
Prize: X=14051, Y=8337

Button A: X+22, Y+73
Button B: X+58, Y+27
Prize: X=5732, Y=5618

Button A: X+98, Y+28
Button B: X+30, Y+90
Prize: X=4222, Y=8942

Button A: X+15, Y+77
Button B: X+79, Y+24
Prize: X=7175, Y=6309

Button A: X+31, Y+18
Button B: X+16, Y+49
Prize: X=3035, Y=2755

Button A: X+39, Y+99
Button B: X+79, Y+21
Prize: X=10277, Y=8493

Button A: X+53, Y+17
Button B: X+19, Y+49
Prize: X=2728, Y=4522

Button A: X+41, Y+21
Button B: X+16, Y+34
Prize: X=14184, Y=5660

Button A: X+68, Y+12
Button B: X+92, Y+79
Prize: X=5724, Y=3709

Button A: X+26, Y+61
Button B: X+67, Y+32
Prize: X=14375, Y=1355

Button A: X+92, Y+19
Button B: X+31, Y+56
Prize: X=10363, Y=6356

Button A: X+51, Y+73
Button B: X+35, Y+11
Prize: X=7960, Y=18468

Button A: X+17, Y+68
Button B: X+88, Y+61
Prize: X=8923, Y=9502

Button A: X+54, Y+11
Button B: X+22, Y+51
Prize: X=11214, Y=9167

Button A: X+89, Y+32
Button B: X+24, Y+47
Prize: X=5467, Y=5726

Button A: X+74, Y+40
Button B: X+19, Y+45
Prize: X=2097, Y=5115

Button A: X+29, Y+83
Button B: X+65, Y+11
Prize: X=19287, Y=12213

Button A: X+26, Y+61
Button B: X+34, Y+13
Prize: X=4420, Y=18770

Button A: X+24, Y+13
Button B: X+44, Y+93
Prize: X=4164, Y=7028

Button A: X+56, Y+11
Button B: X+38, Y+81
Prize: X=1058, Y=2074

Button A: X+20, Y+84
Button B: X+87, Y+92
Prize: X=3562, Y=7852

Button A: X+81, Y+49
Button B: X+18, Y+41
Prize: X=2061, Y=2391

Button A: X+17, Y+48
Button B: X+67, Y+22
Prize: X=14697, Y=17294

Button A: X+67, Y+14
Button B: X+18, Y+74
Prize: X=19074, Y=12106

Button A: X+85, Y+99
Button B: X+77, Y+24
Prize: X=8547, Y=8247

Button A: X+25, Y+48
Button B: X+95, Y+34
Prize: X=3305, Y=5010

Button A: X+92, Y+91
Button B: X+89, Y+11
Prize: X=9014, Y=4140

Button A: X+87, Y+71
Button B: X+18, Y+77
Prize: X=9177, Y=11851

Button A: X+52, Y+24
Button B: X+22, Y+37
Prize: X=10242, Y=17195

Button A: X+40, Y+72
Button B: X+68, Y+25
Prize: X=3604, Y=3273

Button A: X+49, Y+14
Button B: X+14, Y+45
Prize: X=7473, Y=3342

Button A: X+54, Y+15
Button B: X+14, Y+71
Prize: X=1508, Y=16946

Button A: X+45, Y+19
Button B: X+45, Y+82
Prize: X=4950, Y=5240

Button A: X+25, Y+46
Button B: X+41, Y+15
Prize: X=10731, Y=7434

Button A: X+42, Y+31
Button B: X+11, Y+48
Prize: X=3253, Y=4754

Button A: X+73, Y+38
Button B: X+41, Y+67
Prize: X=3365, Y=3304

Button A: X+63, Y+17
Button B: X+15, Y+57
Prize: X=9383, Y=9977

Button A: X+64, Y+31
Button B: X+32, Y+62
Prize: X=13696, Y=16297

Button A: X+83, Y+95
Button B: X+93, Y+20
Prize: X=2813, Y=1145

Button A: X+76, Y+18
Button B: X+36, Y+38
Prize: X=8048, Y=4824

Button A: X+79, Y+63
Button B: X+13, Y+37
Prize: X=3827, Y=3771

Button A: X+22, Y+65
Button B: X+75, Y+31
Prize: X=4996, Y=7118

Button A: X+55, Y+26
Button B: X+14, Y+45
Prize: X=9220, Y=12416

Button A: X+14, Y+48
Button B: X+79, Y+35
Prize: X=5081, Y=12347

Button A: X+14, Y+40
Button B: X+69, Y+55
Prize: X=1416, Y=2340

Button A: X+34, Y+92
Button B: X+47, Y+16
Prize: X=4347, Y=4536

Button A: X+15, Y+55
Button B: X+35, Y+12
Prize: X=19540, Y=3303

Button A: X+17, Y+29
Button B: X+57, Y+31
Prize: X=7619, Y=13715

Button A: X+31, Y+56
Button B: X+43, Y+12
Prize: X=11684, Y=15260

Button A: X+22, Y+96
Button B: X+75, Y+35
Prize: X=9109, Y=12567

Button A: X+28, Y+40
Button B: X+67, Y+27
Prize: X=7819, Y=5879

Button A: X+32, Y+57
Button B: X+39, Y+13
Prize: X=3354, Y=2925

Button A: X+45, Y+13
Button B: X+34, Y+69
Prize: X=2162, Y=8946

Button A: X+63, Y+42
Button B: X+13, Y+41
Prize: X=18869, Y=1775

Button A: X+19, Y+69
Button B: X+63, Y+11
Prize: X=15072, Y=6170

Button A: X+75, Y+24
Button B: X+33, Y+61
Prize: X=5442, Y=5474

Button A: X+13, Y+47
Button B: X+75, Y+23
Prize: X=4656, Y=15696

Button A: X+16, Y+60
Button B: X+64, Y+14
Prize: X=7024, Y=11470

Button A: X+48, Y+18
Button B: X+26, Y+46
Prize: X=9776, Y=8036

Button A: X+59, Y+30
Button B: X+13, Y+39
Prize: X=17579, Y=7712

Button A: X+18, Y+60
Button B: X+71, Y+34
Prize: X=16723, Y=8394

Button A: X+71, Y+15
Button B: X+11, Y+76
Prize: X=15820, Y=5524

Button A: X+50, Y+15
Button B: X+17, Y+63
Prize: X=11692, Y=12383

Button A: X+11, Y+44
Button B: X+61, Y+36
Prize: X=12390, Y=18312

Button A: X+13, Y+32
Button B: X+36, Y+22
Prize: X=17224, Y=6774

Button A: X+12, Y+39
Button B: X+74, Y+28
Prize: X=3132, Y=9154

Button A: X+43, Y+62
Button B: X+66, Y+26
Prize: X=3406, Y=1522

Button A: X+49, Y+12
Button B: X+19, Y+56
Prize: X=2982, Y=6460

Button A: X+33, Y+43
Button B: X+55, Y+22
Prize: X=3069, Y=1913

Button A: X+89, Y+23
Button B: X+45, Y+63
Prize: X=12058, Y=7534

Button A: X+12, Y+65
Button B: X+53, Y+14
Prize: X=16956, Y=18034

Button A: X+49, Y+14
Button B: X+19, Y+33
Prize: X=2815, Y=3480

Button A: X+11, Y+31
Button B: X+55, Y+13
Prize: X=2838, Y=3312

Button A: X+42, Y+19
Button B: X+18, Y+30
Prize: X=6962, Y=14988

Button A: X+58, Y+31
Button B: X+15, Y+38
Prize: X=13711, Y=7138

Button A: X+13, Y+37
Button B: X+43, Y+11
Prize: X=15157, Y=2661

Button A: X+92, Y+32
Button B: X+34, Y+77
Prize: X=4994, Y=2845

Button A: X+22, Y+92
Button B: X+98, Y+55
Prize: X=9490, Y=11300

Button A: X+21, Y+47
Button B: X+47, Y+23
Prize: X=3139, Y=7055

Button A: X+80, Y+55
Button B: X+20, Y+79
Prize: X=9560, Y=12967

Button A: X+58, Y+99
Button B: X+78, Y+29
Prize: X=11448, Y=10064

Button A: X+44, Y+27
Button B: X+45, Y+83
Prize: X=2272, Y=2945

Button A: X+15, Y+63
Button B: X+52, Y+16
Prize: X=5098, Y=7126

Button A: X+74, Y+33
Button B: X+54, Y+75
Prize: X=9618, Y=7497

Button A: X+22, Y+86
Button B: X+67, Y+43
Prize: X=5632, Y=7568

Button A: X+48, Y+15
Button B: X+29, Y+74
Prize: X=8975, Y=1676

Button A: X+84, Y+19
Button B: X+50, Y+92
Prize: X=4456, Y=3590

Button A: X+33, Y+12
Button B: X+23, Y+31
Prize: X=3964, Y=13530

Button A: X+61, Y+19
Button B: X+16, Y+66
Prize: X=9228, Y=9974

Button A: X+23, Y+82
Button B: X+65, Y+54
Prize: X=4645, Y=7318

Button A: X+37, Y+15
Button B: X+27, Y+57
Prize: X=1639, Y=3029

Button A: X+17, Y+55
Button B: X+51, Y+24
Prize: X=584, Y=404

Button A: X+91, Y+36
Button B: X+13, Y+33
Prize: X=4992, Y=2142

Button A: X+17, Y+29
Button B: X+41, Y+14
Prize: X=10979, Y=10376

Button A: X+14, Y+33
Button B: X+55, Y+41
Prize: X=2090, Y=4673

Button A: X+30, Y+78
Button B: X+34, Y+20
Prize: X=3894, Y=6636

Button A: X+63, Y+12
Button B: X+24, Y+66
Prize: X=17198, Y=2942

Button A: X+43, Y+70
Button B: X+28, Y+14
Prize: X=17495, Y=14172

Button A: X+90, Y+19
Button B: X+65, Y+83
Prize: X=4895, Y=4428

Button A: X+60, Y+59
Button B: X+58, Y+14
Prize: X=8120, Y=4542

Button A: X+24, Y+70
Button B: X+54, Y+14
Prize: X=15830, Y=9650

Button A: X+44, Y+12
Button B: X+24, Y+76
Prize: X=18592, Y=11612

Button A: X+39, Y+50
Button B: X+67, Y+18
Prize: X=4683, Y=1930

Button A: X+79, Y+28
Button B: X+18, Y+36
Prize: X=4290, Y=3120

Button A: X+31, Y+76
Button B: X+97, Y+34
Prize: X=4410, Y=4086

Button A: X+34, Y+14
Button B: X+45, Y+70
Prize: X=18489, Y=1194

Button A: X+41, Y+18
Button B: X+44, Y+75
Prize: X=11774, Y=17855

Button A: X+25, Y+45
Button B: X+80, Y+22
Prize: X=8020, Y=4798

Button A: X+88, Y+13
Button B: X+58, Y+64
Prize: X=4800, Y=4035

Button A: X+39, Y+61
Button B: X+46, Y+18
Prize: X=7143, Y=18541

Button A: X+31, Y+18
Button B: X+27, Y+55
Prize: X=792, Y=7729

Button A: X+12, Y+32
Button B: X+40, Y+23
Prize: X=15072, Y=12498

Button A: X+83, Y+32
Button B: X+23, Y+85
Prize: X=7583, Y=6045

Button A: X+28, Y+73
Button B: X+71, Y+43
Prize: X=6381, Y=7115

Button A: X+19, Y+35
Button B: X+90, Y+18
Prize: X=7101, Y=4509

Button A: X+66, Y+74
Button B: X+78, Y+18
Prize: X=4128, Y=4420

Button A: X+72, Y+95
Button B: X+93, Y+21
Prize: X=2076, Y=705

Button A: X+26, Y+13
Button B: X+34, Y+59
Prize: X=2436, Y=6820

Button A: X+22, Y+14
Button B: X+17, Y+40
Prize: X=8817, Y=670

Button A: X+19, Y+98
Button B: X+99, Y+79
Prize: X=10275, Y=12424

Button A: X+12, Y+59
Button B: X+81, Y+63
Prize: X=6381, Y=5559

Button A: X+18, Y+41
Button B: X+50, Y+17
Prize: X=7448, Y=6884

Button A: X+15, Y+75
Button B: X+84, Y+23
Prize: X=18041, Y=19657

Button A: X+16, Y+69
Button B: X+70, Y+13
Prize: X=10854, Y=2011

Button A: X+88, Y+78
Button B: X+14, Y+81
Prize: X=4668, Y=6744

Button A: X+29, Y+27
Button B: X+11, Y+56
Prize: X=2444, Y=2550

Button A: X+60, Y+19
Button B: X+37, Y+80
Prize: X=2293, Y=4072

Button A: X+63, Y+63
Button B: X+18, Y+91
Prize: X=3852, Y=4655

Button A: X+23, Y+62
Button B: X+72, Y+27
Prize: X=1122, Y=6753

Button A: X+53, Y+20
Button B: X+14, Y+30
Prize: X=15622, Y=13840

Button A: X+59, Y+31
Button B: X+28, Y+66
Prize: X=1485, Y=1447

Button A: X+43, Y+15
Button B: X+51, Y+78
Prize: X=1249, Y=1098

Button A: X+44, Y+28
Button B: X+21, Y+41
Prize: X=15653, Y=19705

Button A: X+29, Y+77
Button B: X+86, Y+36
Prize: X=3098, Y=7264

Button A: X+14, Y+45
Button B: X+68, Y+12
Prize: X=1374, Y=7961

Button A: X+15, Y+54
Button B: X+37, Y+14
Prize: X=13954, Y=4596

Button A: X+14, Y+44
Button B: X+41, Y+12
Prize: X=18780, Y=16672

Button A: X+62, Y+24
Button B: X+55, Y+95
Prize: X=5965, Y=7395

Button A: X+77, Y+13
Button B: X+11, Y+50
Prize: X=16710, Y=17641

Button A: X+32, Y+15
Button B: X+60, Y+98
Prize: X=4120, Y=5984

Button A: X+61, Y+31
Button B: X+18, Y+34
Prize: X=7398, Y=7710

Button A: X+21, Y+60
Button B: X+36, Y+23
Prize: X=2622, Y=3259

Button A: X+29, Y+19
Button B: X+12, Y+40
Prize: X=461, Y=6535

Button A: X+97, Y+40
Button B: X+18, Y+36
Prize: X=7061, Y=4112

Button A: X+52, Y+80
Button B: X+56, Y+22
Prize: X=5708, Y=5638

Button A: X+58, Y+33
Button B: X+12, Y+53
Prize: X=4906, Y=6716

Button A: X+13, Y+74
Button B: X+42, Y+11
Prize: X=7083, Y=1489

Button A: X+49, Y+30
Button B: X+20, Y+53
Prize: X=2240, Y=3939

Button A: X+27, Y+80
Button B: X+36, Y+12
Prize: X=2295, Y=4528

Button A: X+17, Y+75
Button B: X+47, Y+44
Prize: X=5481, Y=8499

Button A: X+32, Y+17
Button B: X+44, Y+89
Prize: X=2112, Y=1647

Button A: X+72, Y+36
Button B: X+12, Y+44
Prize: X=7844, Y=10632

Button A: X+46, Y+11
Button B: X+25, Y+37
Prize: X=5516, Y=4111

Button A: X+20, Y+81
Button B: X+79, Y+78
Prize: X=8202, Y=9507

Button A: X+51, Y+17
Button B: X+13, Y+33
Prize: X=8921, Y=2375

Button A: X+23, Y+22
Button B: X+64, Y+11
Prize: X=3263, Y=1012

Button A: X+30, Y+63
Button B: X+63, Y+25
Prize: X=15062, Y=4683

Button A: X+97, Y+47
Button B: X+43, Y+89
Prize: X=7994, Y=5782

Button A: X+13, Y+80
Button B: X+23, Y+19
Prize: X=2907, Y=9189

Button A: X+33, Y+16
Button B: X+12, Y+55
Prize: X=1769, Y=6378

Button A: X+12, Y+38
Button B: X+50, Y+27
Prize: X=11712, Y=4956

Button A: X+31, Y+16
Button B: X+23, Y+37
Prize: X=17186, Y=8631

Button A: X+14, Y+94
Button B: X+70, Y+33
Prize: X=1722, Y=6755

Button A: X+44, Y+18
Button B: X+43, Y+73
Prize: X=12104, Y=6052

Button A: X+94, Y+14
Button B: X+75, Y+63
Prize: X=8685, Y=6321

Button A: X+31, Y+16
Button B: X+14, Y+47
Prize: X=2191, Y=1666

Button A: X+31, Y+11
Button B: X+54, Y+84
Prize: X=3001, Y=2621

Button A: X+28, Y+11
Button B: X+29, Y+47
Prize: X=356, Y=19232

Button A: X+59, Y+20
Button B: X+22, Y+62
Prize: X=18097, Y=6278

Button A: X+18, Y+74
Button B: X+78, Y+11
Prize: X=10544, Y=4148

Button A: X+16, Y+86
Button B: X+74, Y+81
Prize: X=1184, Y=3830

Button A: X+38, Y+33
Button B: X+24, Y+72
Prize: X=2524, Y=5466

Button A: X+15, Y+98
Button B: X+67, Y+79
Prize: X=1821, Y=5440

Button A: X+39, Y+66
Button B: X+49, Y+20
Prize: X=11232, Y=2884

Button A: X+69, Y+27
Button B: X+19, Y+60
Prize: X=13705, Y=8921

Button A: X+63, Y+34
Button B: X+12, Y+79
Prize: X=2298, Y=6752

Button A: X+42, Y+68
Button B: X+85, Y+36
Prize: X=2795, Y=2188

Button A: X+30, Y+75
Button B: X+59, Y+32
Prize: X=1251, Y=2088

Button A: X+18, Y+58
Button B: X+84, Y+18
Prize: X=5436, Y=5388

Button A: X+30, Y+65
Button B: X+59, Y+15
Prize: X=762, Y=18940

Button A: X+16, Y+33
Button B: X+69, Y+17
Prize: X=7919, Y=3927

Button A: X+11, Y+57
Button B: X+29, Y+13
Prize: X=6348, Y=4946

Button A: X+92, Y+22
Button B: X+63, Y+81
Prize: X=3823, Y=4145

Button A: X+97, Y+47
Button B: X+23, Y+79
Prize: X=4404, Y=3966

Button A: X+50, Y+21
Button B: X+33, Y+64
Prize: X=12470, Y=2195

Button A: X+79, Y+26
Button B: X+13, Y+67
Prize: X=17166, Y=7294

Button A: X+27, Y+91
Button B: X+93, Y+40
Prize: X=615, Y=979

Button A: X+13, Y+47
Button B: X+25, Y+17
Prize: X=2402, Y=4648

Button A: X+13, Y+55
Button B: X+62, Y+29
Prize: X=12634, Y=18091

Button A: X+22, Y+71
Button B: X+38, Y+11
Prize: X=17494, Y=19335

Button A: X+14, Y+78
Button B: X+88, Y+15
Prize: X=2122, Y=7545

Button A: X+23, Y+53
Button B: X+69, Y+36
Prize: X=9921, Y=15759

Button A: X+31, Y+43
Button B: X+80, Y+12
Prize: X=6158, Y=1614

Button A: X+97, Y+55
Button B: X+54, Y+93
Prize: X=6326, Y=9326

Button A: X+74, Y+38
Button B: X+18, Y+45
Prize: X=14284, Y=10063

Button A: X+82, Y+67
Button B: X+30, Y+88
Prize: X=8050, Y=8863

Button A: X+20, Y+99
Button B: X+76, Y+42
Prize: X=7172, Y=4755

Button A: X+15, Y+72
Button B: X+90, Y+87
Prize: X=6975, Y=9675

Button A: X+79, Y+21
Button B: X+37, Y+45
Prize: X=8790, Y=3708

Button A: X+65, Y+19
Button B: X+70, Y+85
Prize: X=5065, Y=3933

Button A: X+20, Y+97
Button B: X+30, Y+19
Prize: X=3270, Y=9408

Button A: X+12, Y+25
Button B: X+44, Y+15
Prize: X=10260, Y=11415

Button A: X+77, Y+25
Button B: X+18, Y+65
Prize: X=7553, Y=14475

Button A: X+26, Y+72
Button B: X+43, Y+14
Prize: X=9252, Y=9428

Button A: X+87, Y+54
Button B: X+12, Y+32
Prize: X=4092, Y=4504

Button A: X+71, Y+96
Button B: X+91, Y+21
Prize: X=5172, Y=6687

Button A: X+31, Y+14
Button B: X+17, Y+40
Prize: X=2883, Y=3306

Button A: X+59, Y+80
Button B: X+70, Y+21
Prize: X=9631, Y=7811

Button A: X+63, Y+19
Button B: X+50, Y+94
Prize: X=9590, Y=10074

Button A: X+57, Y+24
Button B: X+11, Y+59
Prize: X=4717, Y=3889

Button A: X+73, Y+39
Button B: X+13, Y+93
Prize: X=7565, Y=12561

Button A: X+40, Y+90
Button B: X+89, Y+24
Prize: X=11059, Y=8844

Button A: X+37, Y+18
Button B: X+27, Y+64
Prize: X=3784, Y=18550

Button A: X+58, Y+21
Button B: X+30, Y+62
Prize: X=1138, Y=770

Button A: X+29, Y+11
Button B: X+26, Y+33
Prize: X=3740, Y=11782

Button A: X+63, Y+26
Button B: X+41, Y+75
Prize: X=3388, Y=3431

Button A: X+67, Y+28
Button B: X+14, Y+49
Prize: X=3226, Y=2755

Button A: X+66, Y+17
Button B: X+16, Y+61
Prize: X=8654, Y=19116

Button A: X+37, Y+31
Button B: X+93, Y+15
Prize: X=3436, Y=1180

Button A: X+84, Y+63
Button B: X+12, Y+32
Prize: X=17000, Y=2517

Button A: X+63, Y+49
Button B: X+32, Y+87
Prize: X=4254, Y=3495

Button A: X+34, Y+64
Button B: X+52, Y+22
Prize: X=11724, Y=14514

Button A: X+39, Y+19
Button B: X+15, Y+32
Prize: X=2216, Y=19158

Button A: X+11, Y+33
Button B: X+57, Y+22
Prize: X=12512, Y=1398

Button A: X+30, Y+16
Button B: X+28, Y+54
Prize: X=16916, Y=15754

Button A: X+13, Y+62
Button B: X+69, Y+15
Prize: X=5267, Y=2506

Button A: X+48, Y+14
Button B: X+30, Y+60
Prize: X=5046, Y=5008

Button A: X+15, Y+57
Button B: X+66, Y+20
Prize: X=6053, Y=915

Button A: X+43, Y+77
Button B: X+77, Y+37
Prize: X=6386, Y=8308

Button A: X+21, Y+15
Button B: X+34, Y+88
Prize: X=1299, Y=2457

Button A: X+50, Y+37
Button B: X+14, Y+60
Prize: X=5272, Y=8766

Button A: X+12, Y+22
Button B: X+63, Y+34
Prize: X=1949, Y=9568

Button A: X+48, Y+29
Button B: X+18, Y+49
Prize: X=16706, Y=17298

Button A: X+67, Y+17
Button B: X+24, Y+70
Prize: X=1109, Y=9969
"
}

#[derive(Debug)]
struct Btn {
    kind: char,
    dx: isize,
    dy: isize,
}
impl Btn {
    const MAX_PRESSES_PER_BUTTON: isize = 100;
    const COST_A: isize = 3;
    const COST_B: isize = 1;
    fn new(d: &str) -> Self {
        let s = d.split(' ').collect::<Vec<_>>();
        let k = s[1].split(':').next().expect("Btn kind").chars().next().expect("char Kind");
        let mut xy = s[2..4].iter().map(|v|{
            let mut c = v.split(',').next().expect("xy comma").chars();
            let a = c.next().expect("axis");
            let d = c.as_str().parse::<isize>().expect("parse delta");
            (a, d)
        }).collect::<HashMap<_, _>>();
        Self {
            kind: k,
            dx: xy.remove(&'X').expect("Btn get X"),
            dy: xy.remove(&'Y').expect("Btn get Y"),
        }
    }
}
#[derive(Debug)]
struct Prize {
    x: isize,
    y: isize,
}
impl Prize {
    fn new(d: &str, m: isize) -> Self {
        let mut xy = d.split(' ').skip(1).map(|v|{
            let mut c = v.split(',').next().expect("xy comma").chars();
            let a = c.next().expect("axis");
            c.next();
            let p = c.as_str().parse::<isize>().expect("parse point") + m;
            (a, p)
        }).collect::<HashMap<_, _>>();
        Self {
            x: xy.remove(&'X').expect("Prize get X"),
            y: xy.remove(&'Y').expect("Prize get Y"),
        }
    }
}
#[derive(Debug)]
struct Machine {
    a: Btn,
    b: Btn,
    p: Prize,
}
impl Machine {
    fn new(d: &str, m: isize) -> Self {
        let l = d.lines().filter(|l|!l.is_empty()).collect::<Vec<_>>();
        let mut ab = l[0..2].iter().map(|ab| {
            let b = Btn::new(ab);
            (b.kind, b)
        }).collect::<HashMap<_, _>>();
        let p = Prize::new(l[2], m);
        Self {
            a: ab.remove(&'A').expect("get A"),
            b: ab.remove(&'B').expect("get B"),
            p,
        }
    }
    fn cheapest(&self) -> Option<(isize, isize)> {
        let mut last_a = 0;
        for bp in (0..=Btn::MAX_PRESSES_PER_BUTTON).rev() {
            let x = self.b.dx * bp;
            let y = self.b.dy * bp;
            if x > self.p.x || y > self.p.y {
                continue;
            }
            for ap in last_a..=Btn::MAX_PRESSES_PER_BUTTON {
                let x = self.a.dx * ap + x;
                let y = self.a.dy * ap + y;
                if x == self.p.x && y == self.p.y {
                    return Some((ap, bp));
                } else if x < self.p.x || y < self.p.y {
                    last_a = ap;
                } else {
                    break;
                }
            }
        }
        None
    }
    fn get_cost((a, b): (isize, isize)) -> isize {
        a * Btn::COST_A + b * Btn::COST_B
    }
    fn cheapest2(&self) -> Option<(isize, isize)> {
        let get_max = |btn: &Btn| {
            let max_x = self.p.x / btn.dx + 1;
            let max_y = self.p.y / btn.dy + 1;
            std::cmp::min(max_x, max_y)
        };
        let max_a = get_max(&self.a);
        let max_b = get_max(&self.b);
        let mut last_a = 0;
        for bp in (0..max_b).rev() {
            let x = self.b.dx * bp;
            let y = self.b.dy * bp;
            if x > self.p.x || y > self.p.y {
                continue;
            }
            for ap in last_a..max_a {
                let x = self.a.dx * ap + x;
                let y = self.a.dy * ap + y;
                if x == self.p.x && y == self.p.y {
                    return Some((ap, bp));
                } else if x < self.p.x || y < self.p.y {
                    last_a = ap;
                } else {
                    break;
                }
            }
        }
        None
    }
    fn cheapest3(&self) -> Option<(isize, isize)> {
        // a = (x_b * y_p - x_p * y_b) / (x_b * y_a - x_a * y_b)
        // b = (x_p - x_a * a) / x_b
        let x_a = self.a.dx;
        let x_b = self.b.dx;
        let x_p = self.p.x;
        let y_a = self.a.dy;
        let y_b = self.b.dy;
        let y_p = self.p.y;

        let a_n = x_b * y_p - x_p * y_b;
        let a_d = x_b * y_a - x_a * y_b;
        let a = a_n / a_d;
        let a_r = a_n % a_d;

        let b_n = x_p - x_a * a;
        let b_d = x_b;
        let b = b_n / b_d;
        let b_r = b_n % b_d;

        if a_r != 0 || b_r != 0 {
            None
        } else {
            Some((a, b))
        }
    }
}
fn cost_multiple(d: &str, m: isize) -> isize {
    d.split("\n\n").filter_map(|ls| Machine::new(ls, m).cheapest3()).map(Machine::get_cost).sum::<isize>()
}
fn main() {
    let s = Instant::now();
    println!("{} {:?}", cost_multiple(d(), 10000000000000), s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn e1_c() { assert_eq!(Some((80, 40)), Machine::new(e1(), 0).cheapest3()) }
    #[test] fn e2_c() { assert_eq!(None, Machine::new(e2(), 0).cheapest3()) }
    #[test] fn e3_c() { assert_eq!(Some((38, 86)), Machine::new(e3(), 0).cheapest3()) }
    #[test] fn e4_c() { assert_eq!(None, Machine::new(e4(), 0).cheapest3()) }
    #[test] fn e1_c_c() { assert_eq!(280, Machine::get_cost(Machine::new(e1(), 0).cheapest3().unwrap())) }
    #[test] fn e3_c_c() { assert_eq!(200, Machine::get_cost(Machine::new(e3(), 0).cheapest3().unwrap())) }
    #[test] fn d_c_c() { assert_eq!(34787, cost_multiple(d(), 0)) }
    #[test] fn d_c_c_2() { assert_eq!(85644161121698, cost_multiple(d(), 10000000000000)) }
}
