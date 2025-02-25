#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr, time::Instant};
use itertools::{self, Itertools};

fn e1() -> &'static str {
    "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"
}
fn d() -> &'static str {
    "\
wwb, bwwrw, wbr, ub, uubwuwg, gwgg, uuw, rbr, bgurbgub, gbubwru, uugwww, rgr, rw, gr, gw, rur, buuw, gwugw, bgbb, wgb, uwu, ubb, ggrbr, wuuwg, wub, gurr, bugg, ruruub, guw, bguugrg, uwgr, wggb, bw, uuu, ggwb, uwbwur, grb, r, ruwu, ubw, rgg, bwguw, ggu, ruw, gb, grr, uruwr, wuwww, gww, wrgwuu, bgrwru, rr, bwr, rgb, bwb, bur, bbrw, rrwb, gwr, w, wgrrw, rwub, gguruuuw, uur, rrrbbrwb, urb, ubwr, uggwgw, buwgu, wrbwru, wru, wwuwubub, bugbbbub, bwubbr, gbu, ggwr, ggbugu, uuug, wgbw, buurbw, rwww, gwbg, wurw, rrrru, wwggbgr, uubr, bgrr, bwgb, uuuug, wg, wwwu, wbgbgwb, guu, gbbwu, uurgw, gwbuu, uwrbug, rrwbrugg, ubbb, uwr, rrbbbw, wbuw, wugwrb, rguru, ugub, buw, wwruwb, wgbrb, rgw, bgb, rrur, b, bub, rubuguw, brgwg, br, wrw, brbrwg, uwrg, guwb, guubwu, uurg, wgugrwbg, gg, wgubwggu, urw, wbuu, gggr, uwgbwrrr, guwwg, uuwb, bwww, gub, rwgbg, uwugr, gbrb, urr, rwr, bruww, uw, wbwuguu, ugg, rug, wwggu, rwbb, ur, wwr, bgw, bbwrgw, rgbwug, bggug, bbru, wgugb, rbw, rbburw, rwrwr, bbgb, rwb, wu, bgg, gbrwwrur, wgwg, rbbgb, ubuwb, gubbb, rbu, urubggr, wbgbg, brr, wwu, wgug, gwu, uru, urgb, uggbugw, bggrr, rg, wbrrb, bwrw, ubg, uub, gwubwuu, rrrrr, guru, bg, wgw, wguug, bruuuu, rub, ggbgr, grgrwgw, gggurbgw, brwrgg, bb, rrww, uruwrrug, brbr, grggwr, rgwbw, bbrubb, brrw, bbuurwb, rwggbww, uwurggg, wur, brrwru, rrgwr, gurrb, brgggw, wbgwb, ggr, gwbww, wbg, rwg, wguu, wuggr, gur, ubr, bwg, gu, rgrb, wrgr, wwbuwww, gbg, wrr, rburgu, ug, rubu, bbw, rggwb, gbw, wrg, gggwbur, brg, wbubbu, bu, uuwr, rgbwrbu, gwgr, rwgbgg, gbrr, wuu, bbgr, gbgrwgr, wggbbw, bgr, bwubg, uurbbgr, uug, ru, wgu, uuruggg, ubrubu, wgrr, rgwubb, rgwuu, wgg, gbbgwr, uwb, rubgbrgr, brgu, rbbgru, uwuwwg, ggb, bwwru, gbgu, wuw, buubug, brruww, ururbwru, ubbrwu, bru, rbrbr, rrr, wuub, rwu, gubwrgg, ubrbbug, urg, rguug, wbrwb, wrbrg, rbg, urbuwr, wrbww, brw, gru, rrb, urgbbw, wugw, ggub, wwg, bgbwb, wbgg, bubgr, wug, ggwbuuu, rrg, rrw, wuugbrg, bubuw, ugwgwr, brurwb, rggu, uwrr, rbrru, wrb, rrbbwr, rgru, wrgw, wwuw, bug, buruw, buug, bbb, bbwr, ugu, bbu, gwgrr, ugbubg, ggg, gwgwbu, grg, bwu, wugg, wrbu, ubu, wgr, gwrrur, brb, uuggub, wuwbu, wwrbr, ubgu, ggwbr, rurugw, ubrbr, rww, gwbgwwww, rrbb, ugbg, grwwb, gbr, ww, uwbbugb, ubgw, urwr, uuwugg, wwgu, wbrgwg, uwgu, rgwrwurr, wrwbwwr, ugr, urbgg, wuwu, rwguww, wrubg, wurg, wwbwb, rwuwurwg, g, wr, rbb, rgu, bwug, ugw, uwbgw, rgrwu, uwruub, brbrb, gurwr, wwguubw, gwg, wruur, rgwr, urbuubr, ruwguru, gwubr, gwgw, uuguu, brgruw, burrrru, bbwu, rggg, wbrb, brug, uww, guur, grw, bbgub, wrwr, rru, wwbbr, gubwrbrg, gug, ugb, bbrwru, bgwuwb, wbgb, gwrgbr, wgggrbg, grbgg, rrbbu, uwbr, bbg, wggbwr, rgrbbgr, bwrrww, wgbb, wbb, uruwbwr, gwbbwwr, wbw, ubbrbgw, wubrugrr, rrggu, bgu, urbw, grbu, gugru, bbrrbb, wubrrgbr, gwb, bww, gwuu, gwuuurw, rwuubbr, gbgbr, urwubww, rrburb, buwr, gbb, uggu, ruug, rgrrugu, guug, ubgrwu, grgwgrg, grbww, wbu, rwuu, gbur, wrwgw, bbr, rb, uwggb

wuguurbbggbgbgbgwurrggbguugurbwrwubwuwwbuwwbwruu
bwwgwrrururwwubrrgbuwwrrbguwguwbgrggrgurrgbrrubbwuwwgwwbwb
rubrugwgwubgrwgubwbgruwbbwrgbwgrubbgggwbwwwrgwg
rwwwrbbuurgurwuwwrrgbwwrggurbrbubugurrguggbubuwu
bgurbbwrgggrwugrbrbbwrbbwbbbrrwbubgrrwubrr
urwbguguwbuurbgguwrbgbuwggrbwwgrbgwrurbrwwbwr
urgguruggwugbwguuggubbwbggruwwbuguuuurrwguurugwugwbrbbwwbb
uuwubwgurgbwbbwugggbrgrrgwuuguubbgbrwbwuwwrrwgbwrb
gwgrwgbrggbwwgwgrbguwwruwubwrbrgubrbbwrwur
grguurbgwrrwbgrrwbwgbgurgwrwrwubgwrbgbgruguburgbrw
bgbwubuuggbrbugrggruwwuwububwgbggubbwuwuuugwwuwwg
gbgggwrrbwgwrgrgbwwubbruwrwruurrrwwuurwugb
bwgwwurrrgrrrwgwgubwrurbububrbgbgrrgrrubggrbgguurwwbrbgugg
rrwubwrgwwubwbrwbrrrrwuuggwwbgbbwwrrwgurwgbggggbwggrwrrug
bwggbrrgruwbgguburrrggrwbrubgwrbgugwubrgrruu
rrwrubububbwurgubgbuwwwgbrwbbgrgrbgwggurrubrwwrrrrrbbw
brrbgbbrwgwbrggwgwwrgwbwuwbugwgwubrwrbuurugru
bbbgubrwbuwgwbrwruugbruwrwbwrubrrbrgwgbguu
uggrbguwbbwwubbgubuggbrwbgbuwrbgwgwurgrbgrburgrrrw
uwbugbgrgrrwurwguruwrwwurrwwgbwuuuwrwwgubrbgguububgwgww
gubrggbwwbguwuwurguwgwbwgbbgbuwrgrruuwwrrbgugggrrwbruw
wbrwgbgggbbrgububggurburgurggubbwwrrgwrguwbubgurbgubugg
wbrwbrbgbwwgrgugrwuuwbwbwrgwurbrrgurgwwruugbbbururg
uwwgrrbgrubgwubugugwguwuuwwwuruwwuwgrgwwrbuwgb
wrbururuwgugbuguruwbgbgwwrwgrwwwwwbrbugwbwwwbgugrrgwu
rgwwwbrbrburwrbuwrrrwbgwuwbubugwrurwggbuu
buurruuwuwuuwbuwuuwuwbrrbgubugrrwuuggbgwbwgrrr
bgruuuggbgubwgguruuwgubwuwbbuwgwburubgruwrbrbwwwuubrwuuur
ubbuwbrrggubwgbgrgbbbrbwwwwbgugubgurwrrgrwbrrurrbrguwrbwwg
uugwugwbrwwwubburwwbgrgrgugwwggrbbrgugwuwbwuuugbgwgrgbr
ugrgbubgubrwwwuwrggrwbwubrrwbggggrgrrgrbbrrgbgugw
bbrurubgbrgrburgwrggbgwrgurgwuubwwbuwuwwggrbgbggugbwbgrgrg
gbugwubrubbbgrbugwgrururggubguwuruwrwubuuuubbw
bbbrrbwwuruubwuugrrbwbggrrrugrwgwwrgugwwbrwgrwwbgwgggwg
bbugbwrbbbgbwuwgwgwuubuwrubrwrbuugubbbuwwwgwubruwrruggwru
grrwgrwgwwwuwuuwbrwgwbrruurwgwwwuwgwwrrrggwubrg
bbwuwbgwbrrgurbuggbwrggwgurgwuwwwwwwrbbwrrbuuugubgrgwr
ubwrbrrwwrrwwrrbwbuwubgguwubwgguruuuwbgwbbbgggbugugu
gwbbwgbwrgrrgrubwrrwbrugggwgugurwbbgrwubrrubbggbwrgubuwbbuu
gbwbrgggggwbrrbugggrurbgwuubgbwwrrbrurwgguuubgbbwuuuuu
rburrwuwguugbbwguuuurgbbwrggwggugrrgwrwbbb
wugwuurbbwgbguwuwggggwuuwwgwwbrugurwrgwururbrwurwbrgu
bgwuwrwubwruugbgrrwuggwwruuuruurubwggurruwuwbggbggurugu
burrwbbwgrrbubggwggrgugwugubgubrwbwuguwbggggubggu
uwgwubgrwrubwwgwbwgubguugwuwuwwgrubuuwwuwggwbbwggrubbwgubw
rbrrwgbwubrwwwburrrbwbbubbrbbwggbguwbbwrgwbuwwg
ubbwguwwggguwbrbrbgrurruugwuurgwwwrwgrwwbgb
rbbbrggggwrggrurwuwurwgubbwbwuurbrgbgwgruu
brurrwuwwrbbuuwbgubgggbgrrbbubwuwbbbbuuubgbguwwbugbrwwbrr
rgrwrugwgbrwbuggbuuwrgbrrrrrbbrwbbbuuruu
gwgurubrbbrwuwuwgbuurwrurrgbrwwuwguurwwgbrubbbbuurwwrrbg
rrrgbrbuwubugrwurrwgrgbbubbwuuuggbrrgwubgwwrrwrugr
wgrbwwurbgrwruwwwgbbbbgwuggbgruuuwbwwrgubwbugwruwu
wbubrwwuwuwrwuwrrgrbrgbuwuguurgubwbggwugwrugw
bwuburbwwwububwubgbgbgguwuubwrugwguuuugbbguwruruwubg
ggbwwuburbrgubgbwrugwrwugrwwrgbbubbwgrugwgwwrgw
wrugwbuwubgwwbwgwgrrgbgggbguugrwrwwguuwbguwrbbguwubu
rrugrgubbbwrwbubbwrrwgrurubggbwbrrwrgwgbgguuugrwgurwg
rbgbwgwbbrrubuwrrugrbguuwbrwbgwbbbguwggbububggubgbrru
rgruuurugbbugwburbwrwubrgrruurwruwgurwrbugr
ugwwwwrubrwrgrwwbggugugwuubrwbbbgrubrwurbbrbrrbrb
wugrrruruwrwburwgggrgrggwbwgwugbgwbgrgggbwwwrrur
uwbbugurrbrgwrwurrwbubgwbrbgrubggrbwwwwrruuguurwruwrwuu
rbbruggwubbuuguwubrbgbbrbgwubrugrrbggruwubbuu
gruwbwrwuugurggwugwgugbrwrwbrrrgwgggbbwgbrwbbbgu
uwuuruuguuwwwwuwgubggwwbggbbwgrubgbggbubgurguuwruu
rbbbugwrrwbbgwgugbbgrruuuggwgwuwugruwbgrugruur
ggurwuububugrrggbrbuururbgguwugwwgruwurwbuguguwugggrug
gbgrrrggrrgwrbgurrwrwugbbrbubuuwwbubwbbbwbubwubbuururwgwub
bwgurrubuggbrwrbwbubbrwuggwuurwwgrrrubwubguurgbb
rrbwrbwbgrrgwbrgrwgwugugrbgbrwgrwuurbgwuwbguugrrgug
bbbbbwbbgggwbrwwrgbgrbwwuggubbrbbgbubggggbubrwrgrbb
wugggggrbwwwurgruurugbrguuwurgbwrugrbbgwwbrbgbgbrwbgg
buwbrwurbbgrwbgwrbwbugbguggwwguggwuwwrwwrggubbgrbwrbwrwbg
guwrgwrbguguggwrgrbrwbbuurugbbubbuuwgggbugu
gbwrurugwbbugrrruwrrrgbrgbrubbrguuggwwwuwububgggugrwrguurruu
wwgbuuubuubrbuwbgurggwbgrggggbwgbrwggrggrwbwrrbgrrgbr
guwgggubwgrwbggubbgwwwwurwbrrbrbgugbwuwrgruggwuburgbrguu
ruuwwbggbgurgburwuggruggrruwbbuuwgwrgggruurruu
rbbrbwurgbwruuggbwuubbwbggggwruggrbguwwgrbbuwwwbbb
brbwbwggbwrugbrwuugbrwgrwuwurwgwgwgrwgurggubbggrrubwwuww
bgrugurrwrgubwrbggubbggbbbugguwrwgrbrbgurugrurruruuwbrurg
rguwubrbuurbwrbuuwbwbwwwwggubrwubwgrbgbuwrrwgwu
wuurbgrgwguugubrwugurgwuwuubwbwrbbrwwgrbwggwggruwgrbubuu
urrbwbrgwrurbuuggrrrurrugwubuggbugrbbbuwuubbwwbwwgubrgrrg
gwgguuuubugwbuwubbbwgbrgrrrbbggubrbwbrubrw
bbbgggggwbwburgrwbugwbwwwgguurwruuwrwubgubrurgurggrwru
gubwwruguugurugwgrwgrrrurwuurguwururwbuuugbgrbuwuwb
rwwubugrburrwubbwruubbbwwrugubbuurrurrrgruwubbuw
gggwbrgurwuuuwugggbwwbwrrwwubuwwgwwuugugrrbwbuubrrwg
ubbwrrggwbrgrrwgwrwbubbrggwrruuwurgwrrggwu
rwwbbbbgbwubrgrrrrugrbrgwrrurugbwbrurwrrbwubuugwububr
gwuruwrrgggurrwguurrurbgbgwbwurruwwruuwuwbrbugr
ggbrbruwrrugrbbbbbuuwbrrrgwrwruurgrugrubggbbrrurbuuggwubruu
wrurwgrrwbugrrrrbwbubrugbrgugbwugbbwwwwwwgruubugwwgwr
ugbbrrbwbruuuurgrwggbwrubgrrgbggbuugbugrbuurwgrurw
uugguguggubgwgguwgubwubuururwbubuwgrurwgggrbrbrwbguu
wgwguugggwwwbgbrwbrugburwrwwrrwgwruwbggwugggrwgrrrb
wrguwgbbgwubggrubwwguuwuwurwuwrbugwuwubbwrwbbbwuurw
wgbbrggbbwbugbwwwwggubugubwubgugruguwwuwurguurgwwgwg
brgwububgbwrwbwgugbgrwrrwbgbgrubgbuggbrwubrgrbwwuurgrggrbw
rbgugwrbgwgrrugbbrugbbbugwwwgwbwrgbrbbgwgur
wbrbubgrbwbgbuwbgbgubrgwwbubuuguggurbrguurg
bwgubgbuurguwbrbuurgbgwgbwruurbuwbwrgrbguububwwuubburr
uuuwgrrgrggwgrwwrwwwuwuurrgrgubrguwurrbbbubwrguurggwruu
brububugbubbgbgbgwbbugbgrgwbuwwbbubwuuwgbgbuwurug
wggbruburbrrwwrubgrurrwgrbbuwgrbrbggrubbgbru
bbwbgrbbwwwurrbrrwruwrwrbrrguubbgrrwrubwwburbgrrugbrbruu
rggrwgrwubugwgwubbbbgwwuwuubuwrguwrwrbbgrbrbubw
gugrbgurgugbwrbwuubwgggwuwwbuugrbuububuwrwgbwbrgrguuwggwg
ggrbgrgbuguwbrgrbwbuubrrbrburugubuugggubwgwbgbwwrwbgruub
bubwbwbwgwubgrrbgruurbgwwgwgrbggguwwububrwbg
rwgbbgubwubuugbbbugrwuurrburgrwuuugwrwbruwggrrbrbrwrwrub
uwwbbbwbwbguguuurruwbgwurbrrguwgrrbugrrgbu
gbururgbrgbwrbbwwgwwwubbwwrbwbruggburwubuurug
wggbugrgwrugguggrwwgbwgrrrrgwgbbwbugrbwgbrgr
urrbrrbwurruburubwugwbbrgbgburgbbgrbugrrrrbruu
wguwbgubbuwwrgggbrwbrrrbbrwbwwbruuugbgwwrrubwuwubbrguurbg
uruwrggubgwbrgbgggwgwugwrwwbwbrbrburrubguugrwwggbuggg
rbuwrrugwbubugrbwuubwwrbguwwbrwwbrgrubguburwbrrbbubgbrg
ggrggurgggrwbbggubuuwwugubwrbrgbwrgwggwwguguugbbrbrwuu
gurbggbbbbubbrugwgrurbwgbbwbgwrgrgbbrrwwbw
gugrrwurwwwwubbbrbbgbwwugwrurrgwgwuubrgbubgbgubruubgg
wuuwgruwrbuwuguwrrwwwgubwrrrggwgggwrrrgrurrbbwb
gwubbbbwgrugbwbgbuugrbrgrgrwguuuburbrbwgrgugruruu
bgugwugrgbruruguubwugburuuwuwwbuwbwuggrugbrbugurw
ggrwrubguruggrwgrbbbwuggbgurbubbguuwrbgugguburgw
wbgrrgugwuubugrwwbrrrrrgwgubgrbgrgguwgwwwrggrgruggbbwbwrru
uurugwruruwwrrubrbrwugrggbrbwwrurrwbbwwrggrw
gurrrwrrwgrwrbbrrbugbgwguururwrwgwbgubrbrgw
grbguwgrbgwruwgrbwwwwwwggwbrurggwbwurrgrrubrwwgwburwbbbgu
buubwgbbuwgwgggrurrgruwugbuuwubwugubwgwwwrub
bgbuwgbrrgrbbrwwwbwuwuwwwgrgruuwrwgrrbuu
bbubrbbgrrgwgbuwubuuwbwwgwwwguwuubrrwrrrbgrwburuuwbrrrbggw
burgbgbuggrburgrrubwwubuubwubwggrgrrrurgrrwgwuubr
buwburubbbwuuwrgrwwugwubggwuwrgrrwwrbbwbwwgrgurgbrurbubw
ubbbgubruruwgrwwgurgbgggbrbbwgrwwbgwrbgwggbg
uwgwgggwrgguguuuwgwwgwbbrbrrbubgrbbrbburuugwgggbwg
urrwgggwgrgbwgwurgrbrugurrubrwrguugrwbrwrbr
ugbgwbrbwwuuuwwruurruuwwgwgbwwgbuwwrruwurgrwbrrwrbubuwuwbr
guwbrgrgbggggbwwbrrrgbgrgugrbwrugrwuurbugu
bgwrbwrugurwuwwbrwgbgbgrurgugwwwwrbburuwwgugg
grbbwgugubrrurwwruuguurwuwwrguuugwrrgbwggrbrurbrbwbwwurrr
uuugubbubbwbrwgbwgrgruwggbrrrrbbwggbgwgwggrbruwwg
ububurwwggwuwbgbbgrbuwrgbggwggubgurwrrbgwrurgrrgurburugrgg
uugbwgrrrgrugbgrgrwubbgbrrgrrgwguugugwwurrugbwbrwwuguugu
gwrwrrwwbrbbbgrwgbwgbrrubbruubgbrbgwrwwgru
rbwgbrgubwwgwugwbgwwwwbgbwgbwbwbrrbbrbwubbrgbuuwgbruu
ubwrwggwuggwrbuwuubguugwrrwbwuwugbwuggwwrugwbrwuwwwbgu
ugwubrbuwruwburggburrubbgbrgwwuuwwbwgwwbwb
bgwgrbgrguugwgubbrrwwrgurbgugbrgbugwrbwgugubrbububwburb
rbwbgrwwgugwggruubrbubbuggwwbrrbwrguwuurrgwguguwbgbrwwbb
wuuubbrbugwwbgrbgbuubrbrrgbguwbguggwubgruuwwu
wbgbwubgwbbrggubuwwwrwbrwggrurburururbbwgwbgwwwwbgbugbwwb
brurburwrurrurwwurugwgbrguwwggbbuburrgwbugbgrgbww
wubgwgurwwuruwggrrbbbrgwubbbuuwwrwrwwwwuwugurwwbubgrwwguwu
rrwgbgrrruugrggwrburwgrrwrguwrugwbgrbuubwuuguwu
grguggugurggwubrwrwguguwbuburuwbugurrugwrgwb
brguwwrbbubwwgbbrurrrwurwbburruuruuuwuuuwgbubgwugburgr
brrwrgwrrruggrgrgrrgwrbwwbrbgguwwwuggrbgwrurgwrbrrbgr
gwbbrwrgbwgwgbrrgugrrwubwrrggggrwrrwruwgwbbgwwgwuwrgrwgurb
gugrrubwguubbggwggrubugbggwuwuwggrrrburburwubwuuubub
brbrwrwubwuwwgugbwrgwbguuuguwubwrwwuwrbwbuugwuwrgbg
brbuuubgwrugwuurwurggbwruwrrrwuwuwuuuubuwrbgrrbgwwg
wrugubuubuubgruururwgbrurgbwggwrgrurbbrrrbrguubuuwurrbugb
wgbrgwbubguwurrbwbubbururbbrbbwbwrbbuggbwbruu
buruguwrgrugubwgbrwuggubgruurrwwwrgwggwrgbugguwugrgwuw
rwugbgbubbrbrubbrrbrbrbugrggwbrgwrwwubugrrbgugbwrg
ubgubugbgbwuuubbrruwbuwrbrrgguwgugrwbgrgbgrwbuubruwu
bbgwubrgwruuwwbrwwurrgrbrwrwburbwguuwwrurrug
rbgrbwwuuugbwugubbuuubrrwrrbguurrrrbrrwuwurrbrbbwuuubwbw
uruburwgwwrwrrbbwuwgurgrbwguugwuwbgbubbwbwbur
ubrrurggwguugugwrgubbggbbbwurrwuuwrwwuugruuubuwb
wbruguguurrgrburguuuwugugugrrrguubggrgbbgguburbggwbrguur
rbbguuwrbbbuwrwggwrguugwrruwrrbgwbbwbbwbwrugwrwbbrbbgg
gruubruugguurrrurugbgbwrugwuurgwuuurbgwbgbguwbrw
rbrbubbgwrgruugrgrrrwurgugwggwrrrgwuuuuuwuugwgg
rbwurbrguwrwubrugrrbrbbubgruwbwgwgurwrugwbbwuwbb
gbrbgurrgwbbbwgwrrwrbguuwgwgbburrgwbbbguwgbrbbbuggr
wbrbuguurgbrrggbgbgrrburwgbuggrguwgrbbgwbgwwuuwurbuu
ggbrrrbrrgbbuuurbrgbwguubbwggwbgwwbrwruuuubuubrw
urwuwubwwuguwbgrrbbwgggrbgwuuuubrgrwgwrrruuggrruggbbuwwwww
gbbguwurrwuwrbrrgrrbbrbgwrrbwrwbruwruggurbgwwrgguwbbuu
rwgggrruwurrwgruwuuggwbbrwbggrwwguwbgguurgbrubgu
wwwbwrbuggwgbrrwbwbrrrggwbwururbwrurrgugubwbgbbbrrwbbwbruu
brrrurrwbruwubuwbrbwgwwbrgwgrrwbrwwgwubugubwruw
rwguurwwruuguwgrbrgbwgbrwbbbbbrubwbgbggwgrurw
gbgrwbrwbgrwbuuuwubbububrurguubwguuruwrggbugwbbbrguruu
bwgrwbuwrgbbwwgggggwbrrrwburgrbrburuwwuguguururrwrru
rruuwrgrwrbuwbbgruuggubuguburwwgbbbbugwwggrb
grgwrbubwwbuwbuguubwbbrbwugururgwwguwurgwwwugururg
rgrbwubuwrwuurugugggrwbbbruubbbrgrgbwubrwrwbrgrwrrwwgb
uuwwubbuurubwugwwgubbubrrrgggguuuuwubuwwgruu
urrbuwuugwwgggwubwrrwrrwbruggwubwrbwgrgguurbwg
grrggrgbgwrwbubrurgbbggrggbubwbbuuwrgbrrurgrguububug
urbbgrubruggburwwwggwrbubbburrugwgbrrgubrb
urwwbgwwbguruuwrbuwubrruwubrbbbguuwgwrurgggrbrubrrrwrurug
bbbwwrruwugbwwwwubuguwubguugugwbbwubuugururwuurrwuwuwgbgr
rwbwwrugrurggwbwggrwgururbgrggwrwggugguwrrwurgbrwgrgu
gbburruburrgwwgruurbgbbrbgwwrurrbgwwbbguggubgrggurrurw
gbbwbgggwrwwwwwgugwgrrwwrrwurgubrugrrrrwwuww
ruubgwugbwrurbrbrrububwuguwwbrbwwgbrwubrgrwuwrurbbrruggwur
rburgruubuwbwwrugrggbbwwrwwbbwwrrguburuwurrwuugr
gwrwguggwrrwgugwbrbrguwwgbrbgbwggwwurbubwrugggbu
wgrwbgrgubgwwbbuubggwwgbbbuuuwrwrwubbbwwrrrgwrurwwrurrgw
wwbwubrrurbwurwbrurbrrgrgbrgwwgwubwguwwwuugrggr
rugbwbuwgwwubwwuuugwbgwurrbrubggwwrrwrbrrwrwgw
urwgbguubwuwwrrgwgwggbbgugrugrwgrguwbgbwbubbgrbgb
guwugbgwbrwguwgruruggwuwwwrbwurbbwgbwrbwugggrrwbgw
bugggggrwrgugrgbgbwuwwrwubwgguwwrbgbubggwwrwwrgbgubbwgbr
rrbgwrrwgbbwbrbrgwrwurrbwgbwwgbgrgbgwruu
gwuurrgrwgwrbbrwbwruggbuwwgbbwrrgbwrgwgurwgrubgbugwgbgu
gbrrrgbgguuubrrbbbwrwbuwwwgwbbggrbggwububrwbbwgrgrwgugwb
wwwrbbrbrrbbgrwwwrurbrggbugrrwbwwbwgwbgggwrugwgguubbrwuw
uururbwrugrugrrbbuugwgrbrbrgwwwubwuuuggbwgwwwubrwgrgbr
rrrguwwwwgbrwbbgwwguwwbwgwuuwugubwggwggrwrwbuw
rrwwrurwgwubrrgbrrurwrgwgubrrrubguruugwgrrb
rwggrgwgubruurbbbgwbgrbugugwbgbrbbugbwrbwbbrrguruu
wbwbrurwgwurgbugbbbwgugwruguwwguwwwwugrbbubuug
wgrgwwgwbrubgbgwwwggugrrbggrwbwrggurbuwgugbgurrrruuw
ggrbbbwurggrbubugbbrrubgrwrbuwrbwuwgubguwwg
gurbgwwggwrguuurggbbbgugubbgrbwwuwwbwrwwrwuguwbbwww
bbrurwwgwwwuuggwwrgbuuwgrgugbugrbgrgggburubbuwggubwb
bwwwrgrbwgurrwwuugwrrrbrugrgrgbrgrwbbgrurubggbgrgwb
uurguwwwbrrbbbbrugubuurbbbwwgbuwwguwurgubgbbbrbguuwuruuu
wubbggbrbguugrbrwrurrrwbrbruwgrrrbrgwbrbbugbgwbbugbuw
gurrwrgbbbggwwwubwwgurrugrgrbgrbgrruwuurbbg
rgrrwgurbbbugwrbrgrurrrbrwrgwuwbburrbgguuuguwwubgw
bubrgugrbrwbubggwrurbbggrugrrwrburubwruugbuubb
gurbbwbwgwruuggbrbrrwgbwbrububurrwurugburwbrgr
wwbuuuggbbbwwbbwugbubugwwgrrubuuwurrurbbrrbgg
uuuruuggurbrrwwbbbggbbwwuwrwrgwrbrwuubuwwwububrwgwruurwu
wwrgbgwgggguwuwwbgrrurwwwwruuugwruwgrwgwgrbgrgrbgru
rrbrbuggrrrruuuwuuubggurbbbgbuuugruwurgbuugugwrugru
rburbrwwuuuwbbwgwbbbwuwbbubuwugugggbrbggrbggugrwg
bbrrrrwubrggwwgubgrgubrwrurrrggbburgwugurggrwuubugrrgw
wwwguuggrwgbgwgugggwwbrburgbuugrgbbbubgrwguuw
wgbbuurbwgrbrrwrugguurwruugguwrguwugrwwrrgwuggg
wgrurrbwgurbugrburgurbugwwbwubggrbbwurbubwug
bgbbubbuwguuwurbgbrurrbrbwruugrbrrgwrubbbwbgrwwbbwwg
wwguuuurgbuubbgbubuwbbgbbwwgrurgrbgubgbuggwubuurbr
wwbgrgbgbrugggwwbbrwuwwuuurrrbrwbugbbbuburruu
guwgrrgrbwbbwrurgrwggbugrububrwgbggwbggbbwguuwb
wbbuwrurgrwbrgrubbbgruuwuubbruwbwwgbbgrgubuugwururuurruu
ugbbubbbbbururggbrburbgurubbbbwgbbgbbuurwwwburbg
grgwbbwrbggugrwguubuwugbbgrwrugugubgrrurbggrbuubbruggru
uuburwbwbugbubrbrwguwbrbuwgubgrrrrrrrbuwgwgwu
guubrurggrbrwbuggbbbwrbrbugguwgbgwgrrrgburgwwrgbgrb
rgurubgbwgwguuugubuuwugguruwgbrgugwgbuwbbru
rgrubwrubrgrrrugwrugwggwbbuwbugwgrbuuurbrrgbwbbwgrwwbguug
gbugrrbrgbwrwbuurbuwgugbuuurrugugrgurbrwbubgruu
rrwrggrwgubgbgbbrrbwrrubgrgrbwurugwubgrbgubwr
gwuwbwuuugbwrurwuwugwwgwuwwbugbgurugwbgrgb
grrrbwrugguruuuwgggwrgubgbwwuuguwbbbubbrburrruu
wgwbwwrgguwubbrrugurbwgwbubrgubugugrubwbrwgurwwwbwgrrwuw
rwrbrgguwrgrrgwgrrgggwurubruugbbruuuuubbgwwwgbuggrgrguwbgu
rbgrrwwgrwrgrwrbrwbubbwrgwrwrburugurwubwgrbgwrgb
bwwwgbuburuwuwwrwgugwrwwwbbbggubwwwbuurbgubrugwubrbww
ugbuuuwurbrwggbwrbgububbubrgrwggrrrrrgbgubwwruububggur
uuuuwrbbrburrrwwgwuurgurgguwgwrrwurwruuurbrggw
rgubgrbwugbgrrgrbwguuwwuwbuburbuwggbrbrwggrwwgw
bwrwrwuggbwgwuwwwbrggwrwuwwbgrgwuuwubbbruwwrbuwrwbwwwrurbu
gbguwruwgwbbggrwgbwrubgbbbwgubrbuugwwrbwwgrgwrguuuugrurru
uburbgbgbgwbrgrgubrbuuggbgbuwurwwrwrubwbuu
wwubuwguuwrbgrwwbbubwgwwggrwbwbuwwbubrwbbuburw
grbguwuubggrbuuwgbrbbbuwuruubrbbubrwubrrruugbwuugguggg
gguwgrbwurbuwwruuuuuwgrwbwgbbwurwubbrbrwggrbrrb
gubwwbwguwrbrrwwgbgbwwuuwgrbgwbwburbuwuwburgrgbrb
uwgwrgrbugubbbbgbbwggwwbwuwrrurgwgubwurgubuuwbggrwgwrgbu
uuuguwwwrggrguruggwwgwwrububggwrrrguuwgggbrugbbuu
ugrbburubbgurgwwbbubbrububrgbbrubrburbbwbgrrrwggubggugruu
ugbwuwugbwwrbwubburwbgrwwgggruwuwgggrubwubrrgbrruu
brbggbggbrububgggwbubgbwwwgbrrbwrburgrubggrruwwbuubww
rwbgbrwggbrruwbugrrrruurrrbrbggwgbbbrbwwwgrrrugbrgguur
ugurgbburrgrrrubwrguwguuruggguurubgrrurggbrguu
buwbugbugwguruwrwbwbgbbgbggugrubgwggbwbwbbgbubuurr
rrwwruuuugwburruwuubugwggbgwrbbuuwwrrwrgwrgbruu
uwrgbrrrrgbbbwrbrgbrwwrurbugbgbrrbrrbuwubuu
bruwgwrwgrwbubwwgubrwbuurwrbgbruwuuwgbgwuwwuwrrgrubw
wugguuurwbburrbwbuwrrugbuugwwbbgwgwubrwwbbubrruurbww
rggrggwbwuwurwrwwrrbgwubwwrbgrrurbrgugbruuugurbgrrrbwu
rwubbrbbuugubugbbbubgruwgurwguurrbrggubrbbgrwbgwgugbrub
ggwwwwuugbbrurbrgurbbgrrwgwwwrwuwuurbbubbug
gwgugubruuwrbuwgbwrrruwrrgbwgbwwrrggrrrgguwruruu
bgruuuwuwrugbwgugwwruwbwguwrwuburrwgbgrggwruw
grwwgrrwwuugrwwuuwbgwrburuwrbwrugwwwrrbgguubwwub
bubrrrwrbuurbwbuugrgbburwbwurubuuuburubgbbwbrgwgbbuu
gburwwurwrrguwgwuggbururrwggbrurbgbgbbgggurbgwwwbubbuwuwg
bgwwwuuwgrgwwrbubrwgbbgrruwrbgruuubrbwurwwbuwgugrwbbgwuw
rwbgbbrggbbbugrbuwbrwrbggwggwurbwwbgbubgwwbrubbubbbwgwurgg
wgubwggubgwrbwwbgbuwwwrruubwrrwbwwwwrrgburwbwrbrgwbgwrwuruu
urwrwrwrbrwurgbwgrgwuggbwrruuwgrgrrwgrwwrwg
bwbuwguguwggbugbrubwugbwgbbrurugguuwrubrwrugububwubbguwrrw
urrwbbbrguwrguuwrwrubggruguwugbwgbggwgbrgwwrwrrwrwubuwuburuu
uwgbwrrrggrgwwwuwbbrgbrwwbrwbgruugrbwrgwuru
ggwuururwwwguuurwgbwrrgbgwwugbrrugrwbwrrwubbubwgugrwbgruu
uwwggbbuwggbruurbrrgwrbugbgwuwwggwguruggrggubrrbbbgrgggr
wurwgwwrrugwwggrbwrgbbbwgwwubuwgbrgbgwubwbubrurbuwguwr
ggubrwgbugwuwrruwurrwggwuwwurbwbwgwgrwugbgwugggu
uuububugwgrguwgbuwurgbrgbgrgubwgrbwrrurrubuu
wbrrwurubuurwwbbuwbggbrrrwgurggguurrgbgggwwb
burbubrbuurwrwbbrwugrgbburbruubrwbwubwrgbbrbrbwubruu
ubgrrwubbwwggwgggugurgwrgbguugrrrbgbuwwgrgguuubrbbuwwwrggruu
grwrrwuwwwrgwgrugguuuwbggugrgwwwwggbbbwwbugbruu
gbwwgbrwgwrgbwwwbwubuugwrbggrbguwgrgwubbuwuwubgguwubw
uuwgrrwgubuwggwugubrbubggugbwubgwwuugurbbwuugwugwwuggbgbu
rurgwrrgrwwuuwrgugwrrgrubbrbrgurbubbbubwrwbgrr
urrbuuurrrwbgruwbwurwgwbgwgbburwwwbrgrbrgrwugwubgwbwgrbrrg
rwwwurrrubwwruwuuwgwwugwgbubuwggbwugrubgubwwurbbuubuu
gurgruuwguurwbwbugggwuwubgbrrubwwrrbggwwgbrrwgur
rguwwruurgbbbrruwuwwrwrwrrurrwuwuwggubuwwwugruuugwgbw
rwubbwrbugrwrbwbuuwwurubgrrwwuuuuuruwuwgwwrrwuwbugrgbrg
gwgubbgbrbbwbgbubrwrbwwbbrwwrggwggbwgguguu
gggwgububbbggguwuuwuwrubgbrbrbgbgugbuwgbwwgurgb
bgrrgbbwrgwruwwwgrbrwwwrwburwgbrwubbggruu
brgrwbrwrbgruwwwgurwgrwwbguggggbwgururrruu
rruugubwgbugrgbrbwuurbubgrrwgrrrrguuurbbbrwbwburrgwrwub
bbwgbwugugbwrrwrbbuggwbguwwwrgrwugwbburwgbbrur
rgbrbwrwgrbggrguurubuubbwgbwbbrwugwrgrugwwbugubbrwwbwrrw
rurbrwrbgbuwrbugrgubggbwggbbguwugbwgrwuubrbwguurwwubrwurbr
bgrurwuuurwbuuggbuwruguurwwbbgugwuuwgrrbbwgrbgwuuwruwbwr
gugbrrrruwgbrugwruubgrgbgwbrwbbbbuugrwgwuuwbbwwwgbbgwbrg
ugrwgwwruugbubrbbgwggbgrrurgwrgbrwbgugbubbuguggggw
bbrwurrgwgubrrrwbruwubbwwbrububuuwurgbbbwwbrguuwurwuru
urbwwwwbwrguuwbgurwurbrubugugwbruruwwwrbgbubbggrrw
wrwuwbwbbwubrugrgguwbrggwwwrbwwgwubggbrgwbbbbubguggwbr
buwgrggrwwbrwwwgwwwwubrurbgwwuururbuurrwugrwrgguru
bgugrggwurrurgrbbwguwbrgbgwbbrwgbwguwuuugbruuu
rrgbuggburwbbuuwrwbgugrbbuuwwwgwgbwuugwubwrubrwurgubbwurb
wrubgbrgruwgwrguwrgggubrgbguwuuwuwbrgrgrrrwwuuwrwwwuurbuu
gbguuubrbwubggurrwbrwrwrbrwrburburwguubgwbbubbuubgbwruu
uwrrwbwwgbuwbuwwrrrubwbuguuwgwbgrbguwuuwruggurbuguruu
bubwuwwwbbbrwrgubugguubrruggugbubbuwrubgrwwrgwurwrwg
rwbburbrwbbuguwwbwbwgrurwbugurwbggguuwugrruwwuwubbugguub
ggwbwbgrggwgrurguwguubgbbbbrurgwugguwbwugwbgubwwru
bwgburuubbgbuubbuwgrbwbuuwwwrbgugwrrurgwugugrrg
brgwgwbgwrwggubrubrbgubwwbrugburbwugwwgurrugwburbbgbw
guubbrggurwrrugrbbrbuuuurgbbwrrubwbuwrrgbuwrwwu
rrbuwuugrrwguuggrwugubgbwuwrbrwbugbrurggwwgwbr
ggugwbgbbgrbrwrbbuuwuuurwruurggubgurbgubwgruu
rwgrwrrgbwggwwrwwwwbruwwwgbrrgbbubgbuwrugwwbbbbwbrgbgrgr
gwwruugwrrgruwubbgwuuugbwurggwbbgbuuugwrgwuugbu
ruwuwbrrruubrgbbuuwggwuwgguwwwburgrwugwrurgrugurwgwgubrrw
gubrbrwbrbbggwrburbuuggrrugbbgurwgwgwbrgrwggwrgwrwbrww
rbbwwwuwrwgguugrgubbwwrrgruubwrrwrurrrrggrruugwur
ugwbrgbbrbgbuwrburbrrwbgubrwgggwgbrrbbgbbuwwrrwubgb
wugrubgrbbwrwwwrrbrggwrwgwgruruwwurgrbwbwgbuwrrggr
bwrbubrguuwbgwbwgbrbbbbwbgbgrwrwbbuguuwwwuwgwrbuugub
wbugbbgrbuuuuugurgwwbrwugugrwugwrgwbrrguruubuub
rrwuruuuruwuurggurwwuguwbgburgrrwbgugbguwuurbugwggwuw
rrbrgruguwbgbuugugrrrguurrgwurbgwwwruwrugbrggwgrgggbuw
bgbbrbbrgwwbgwbrguururwbrbwbggrwgrubbbbrgwuruu
rgwgwwbrwbgruwrwubgbruurwbugrgguuwguruguwubbgrbwbbwbw
buuururbuburuwbuubggwgwbrrbrgbubgwrubbrbgrrwuwuwgrwgrwww
gugruwuwwwwwrrrrbrbbgbrrrbrwbrrwbrrguubbgwubwub
rurgrrrwwrgbwwruwgrrbrburbgrggrruugrwgrurwrbugwrg
ubgubggwrggbwwbuugrwubgbgwgwruuwrburgwbrwrgbrgruuuwruub
burubgrbbrwgburguugrbubwurubrbbuwbuwrugwgb
rwbuwbrgbbwwrwgrubwbrbbuguwgwgwrwrrrruguubr
bgwrwuurgugwwwbbuwrgurbggrgugwbbwwruguurwbbbbbuwgrgrgbwuw
burggrwbrgbwgggwbuuuwgggggwbgrugugrubuwggrbrbrgwwbbbug
wrgbruwbrbwbbbwwrggubwrbrgurbwrwbgwwrrgwgurbrwgubuu
uwwwgrbbburuwrrugwugwuuuwbgwbwbwrgggrbgruu
wbubwrbguurbbgurrguuwuwgrwuububwrgrgbgugwbg
ubrrbwugruugruuurwggwrbwgugbwruugubgbuwuurbwurrruwubwgg
ggrbwubbgrgwbbbgbbwwuwruwbugbbuuuurrburrrbwgbgwwwuw
bbgrgguwgbgbrwbwuugwrwgwubggwbgguuubrrwwbrgwruugg
rgbuubrrbgwgwwuwwgrbgbbgubuugggwrrbbburrrwwuwuwbu
ubrrwrbuurgruguwubgbugrbbgbbwurrrwwrgbwggrrggbubggg
buwruuugbwbwrugbburugwubrrrgrgbbbrbwgrrrgbggg
wgwggurwbbwbrgwgrbwbrgbbgwbbwrubrburrugrwwrbugbggwg
wgbrwwrurruwrbbrgrubuggururugrwgggbgrbgwbrg
wgrbuwbwbrbwrwwbrruruugwrwrugwgrwbrrwrbwwugggbbg
rbbuggwwrubgubbbwurbbgwrgrbggbgurbugrgruggrrwbgrbbrrubgwr
gwgwwgwwbwbburwguwuurggwrbgbgwrwbrgwgbuggr
wubgrrurwwbbbrrwwwrggwuuguwgggrgbbbgrwruurbg
urwuugbuwwwrrburbbubrgwrrrbbgwurwbgugbuubwbwgw
buwgrwwurubwubgrgurbbwwbwbgrbrrwurbrbbbugwrrrggwbuu
wurrbbuwgwuguwgwurbgwbwrbguwbuwgwbubrrbgbrgwrbgwrrgbguuw
urbbggbburuwrrugruuwuwwwuwgbwbggbwrgwgrubwrb
rwgrwgrubrrrwbrbbubbgbgrbwbwwgbggrggrgwgbwgbbgwgrgbbg
bbbuugwguubgguwwbwurbwwgubwggurwurbwuguwwbuggrwwugburwbru
bugbwubrwrbbbbgrugrwgbuwrgggwbwwwgwrrwrggggugggruuuu
ggbrwwwbuubuurrguuwbuurbrwuubbrbuwrugrruurrurgu
ubgwurrbrbbbrbuuururrggbbrrggbwrbwubwgbubugwwbgbbuwbg
rrrurgwbwbrbubggubgbwwbwbbwrgguwwwwwrrgruuugbw
wuwgggwbuuwgbrwguwwwruuwwubguwgggurbgwuggbgubuu
rbbrbruwuggrrwgubrgwgruububurwrrrwwbbbwrbbggruguwwr
ugbuubuwwgrwwbrgubrwurwwgubrbbrrgbruugwrwggrb
rrrgugruuuurbgrguwurbbrbgrwuuwggbwubguwrrrwurgub
wrugbwwbugrbbbrwbuwbwurbgguburrgwrbwurubwwwrugwugwruuw
wggurgbrubwbwruwbbgbrbugwbrwbbuwrrgburbugbuwb
rwgbrwuugwrwbuburuuruggbrwgwrbrurbrurrbgggb
uwbggubruwwrgwwbbwgggwgbrurwgrrrwbrwbrubwgwwwbwrr
bbubbwuwrwwruguwwbwrburubgrrbggbugwwrrubrbwbrrgbubgg
ggrbbbuggrbbbubbrbbbgwwugrgrbrubgrrgwrurrrwugrwwwwbuggg
wwbuwgwbwbgwgurwbbrbrwguubugwubuwuuguwbrgwgrugbgbbbubwg
urgbbuuuwgwbgwgwubbwuguuwurrgggugguuubbruwggg
bggwruwwruwwwuruuuwbwguggrguubwgubrgwwggwuruwrgwwubruu
rwbgbbrgbruugbwwwwurwrwbwwrbgrwgrwugruubuwbubwbwgbrubg
"
}

fn parse_avail_desired(d: &str) -> (Vec<&str>, Vec<&str>) {
    let (av, de) = d.split("\n\n").collect_tuple().unwrap();
    (av.split(", ").collect(), de.lines().collect())
}
fn get_lookup<'a>(av: &[&'a str]) -> HashMap<char, Vec<&'a str>> {
    av.iter()
        .copied()
        .into_group_map_by(|&v| v.chars().next().unwrap())
}
fn step_towel(av: &HashMap<char, Vec<&str>>, t: &str, s: &str) -> bool {
    if t == s {
        true
    } else if t.starts_with(s) {
        let nc = t
            .chars()
            .nth(s.len())
            .expect("should have char left since t != s but t starts with s");
        if let Some(ss) = av.get(&nc) {
            ss.iter()
                .any(|&a| step_towel(av, t, &(String::from_str(s).unwrap() + a)))
        } else {
            false
        }
    } else {
        false
    }
}
const DO_POSS_PRINT: bool = false;
fn towel_is_possible(av: &HashMap<char, Vec<&str>>, t: &str) -> bool {
    let s = Instant::now();
    if DO_POSS_PRINT { print!("{t:60}"); }
    let r = step_towel(av, t, "");
    if DO_POSS_PRINT { println!(" {r:5} {:8} us", s.elapsed().as_micros()); }
    r
}
fn check_towels_possible<'a>(
    av: &HashMap<char, Vec<&str>>,
    de: &[&'a str],
) -> HashMap<bool, Vec<&'a str>> {
    de.iter()
        .copied()
        .into_group_map_by(|&t| towel_is_possible(av, t))
}
fn count_towels_that_are_possible(av: &HashMap<char, Vec<&str>>, de: &[&str]) -> usize {
    // check_towels_possible(av, de).iter().fold(0, |a, (&b, ts)| a + b as usize * ts.len())
    //   following seems better/easier to understand
    de.iter().counts_by(|&t| towel_is_possible(av, t))[&true]
}
fn count_towels_that_are_possible_from_data1(d: &str) -> usize {
    let (av, de) = parse_avail_desired(d);
    let av = get_lookup(&av);
    count_towels_that_are_possible(&av, &de)
}

fn count_towels_that_are_possible_from_data2(d: &str) -> usize {
    let (mut av, de) = parse_avail_desired(d);
    dedup_av(&mut av);
    let av = get_lookup(&av);
    count_towels_that_are_possible(&av, &de)
}

const POSSIBLE_SINGLES: [char; 5] = ['w', 'u', 'b', 'r', 'g'];
const DO_DEDUP_PRINT: bool = false;
fn dedup_av(av: &mut Vec<&str>) {
    let print_fn = |av: &mut Vec<&str>| { if DO_DEDUP_PRINT { av.sort(); println!("{} {av:?}", av.len()); } };
    print_fn(av);
    
    av.dedup();

    // part 1 - only include singles and those that aren't comprised only of singles
    let existing_singles = av
        .iter()
        .filter(|frag| frag.len() == 1)
        .map(|frag| frag.chars().next().unwrap())
        .collect::<Vec<_>>();
    let missing_singles = POSSIBLE_SINGLES
        .iter()
        .copied()
        .filter(|c| !existing_singles.contains(c))
        .collect_vec();
    for (remove_i, remove) in av.iter().copied().enumerate().rev().collect::<Vec<_>>() {
        if remove.len() == 1 { continue; }
        if !remove.contains(&missing_singles[..]) {
            av.remove(remove_i);
        }
    }
    print_fn(av);

    // part 2 - remove larger frags that are comprised of smaller frags and existing singles
    for perm_i in 1..4 {
        for (remove_i, remove) in av.iter().copied().enumerate().rev().collect::<Vec<_>>() {
            if remove.len() == 1 { continue; }
            for contained_in in av.iter().copied().permutations(perm_i).filter(|v| v.iter().all(|&v| v != remove && remove.contains(v))) {
                let without_contained = contained_in.into_iter().fold(remove.to_owned(), |a, v| a.split(v).join(""));
                if !without_contained.contains(&missing_singles[..]) {
                    av.remove(remove_i);
                    break;
                }
            }
        }
        print_fn(av);
    }
}

fn main() {
    println!("START");
    println!("{}", count_towels_that_are_possible_from_data2(d()));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn t1_e1_get_avail_desired() { let (av, _) = parse_avail_desired(e1()); assert_eq!(av, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]); }
    #[test] fn t2_e1_get_avail_desired() { let (_, de) = parse_avail_desired(e1()); assert_eq!(de, vec!["brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"]); }
    #[test] fn t1_e1_count_towels_that_are_possible1() { assert_eq!(count_towels_that_are_possible_from_data1(e1()), 6); }
    #[test] fn t1_e1_count_towels_that_are_possible2() { assert_eq!(count_towels_that_are_possible_from_data2(e1()), 6); }
    #[test] fn t1_d_count_towels_that_are_possible() { assert_eq!(count_towels_that_are_possible_from_data2(d()), 344); }
}
