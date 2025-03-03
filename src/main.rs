#![allow(dead_code)]

use itertools::{self, Itertools};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    rc::{Rc, Weak},
    str::FromStr,
    time::Instant,
};

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
    if DO_POSS_PRINT {
        print!("{t:60}");
    }
    let r = step_towel(av, t, "");
    if DO_POSS_PRINT {
        println!(" {r:5} {:8} us", s.elapsed().as_micros());
    }
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
    let print_fn = |av: &mut Vec<&str>| {
        if DO_DEDUP_PRINT {
            av.sort();
            println!("{} {av:?}", av.len());
        }
    };
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
        if remove.len() == 1 {
            continue;
        }
        if !remove.contains(&missing_singles[..]) {
            av.remove(remove_i);
        }
    }
    print_fn(av);

    // part 2 - remove larger frags that are comprised of smaller frags and existing singles
    for perm_i in 1..4 {
        for (remove_i, remove) in av.iter().copied().enumerate().rev().collect::<Vec<_>>() {
            if remove.len() == 1 {
                continue;
            }
            for contained_in in av
                .iter()
                .copied()
                .permutations(perm_i)
                .filter(|v| v.iter().all(|&v| v != remove && remove.contains(v)))
            {
                let without_contained = contained_in
                    .into_iter()
                    .fold(remove.to_owned(), |a, v| a.split(v).join(""));
                if !without_contained.contains(&missing_singles[..]) {
                    av.remove(remove_i);
                    break;
                }
            }
        }
        print_fn(av);
    }
}
fn count_towel_combos_slow(avc: &[&str], r: &str, _step: usize) -> usize {
    if !r.contains(POSSIBLE_SINGLES) {
        1
    } else {
        let avcn = avc
            .iter()
            .filter(|&f| r.contains(f))
            .copied()
            .sorted_by_key(|v| usize::MAX.wrapping_sub(v.len()))
            .collect_vec();
        // println!("{:_step$}", avcn.len());
        // avcn.iter().fold(0, |a, f| a + count_towel_combos(&avcn, r.splitn(2, f).join(","), _step + 1))
        avcn.iter()
            .filter_map(|f| r.strip_prefix(f))
            .fold(0, |a, r| a + count_towel_combos_slow(&avcn, r, _step + 1))
    }
}
fn count_towels_combos_slow(d: &str) -> usize {
    let (mut av, de) = parse_avail_desired(d);
    let avc = av.clone();
    dedup_av(&mut av);
    let av = get_lookup(&av);
    let ts = check_towels_possible(&av, &de).remove(&true).unwrap();
    ts.into_iter()
        .sorted_by_key(|v| v.len())
        .inspect(|t| println!("{t}"))
        .map(|t| count_towel_combos_slow(&avc, t, 0))
        .sum()
}
fn add_towel_combos_abandoned<'a>(
    av: &[&'a str],
    r: &str,
    res: &mut Vec<Vec<&'a str>>,
    tmp: Vec<&'a str>,
    _step: usize,
) {
    if !r.contains(POSSIBLE_SINGLES) {
        res.push(tmp);
    } else {
        for &f in av {
            if let Some(r) = r.strip_prefix(f) {
                let tmp = tmp.iter().chain([&f]).copied().collect();
                add_towel_combos_abandoned(av, r, res, tmp, _step + 1);
            }
        }
    }
}
fn count_towels_combos_abandoned(d: &str) -> usize {
    let (mut av, de) = parse_avail_desired(d);
    let _avc = av.clone();
    dedup_av(&mut av);
    let avh = get_lookup(&av);
    let ts = check_towels_possible(&avh, &de).remove(&true).unwrap();
    let short_combos = ts
        .into_iter()
        .map(|t| {
            let mut t_short_combos = Vec::new();
            add_towel_combos_abandoned(&av, t, &mut t_short_combos, Vec::new(), 0);
            for v in &mut t_short_combos {
                v.sort();
            }
            t_short_combos.sort();
            t_short_combos.dedup();
            (t, t_short_combos)
        })
        .collect::<HashMap<_, _>>();
    #[allow(unused_variables)]
    for (&t, combos) in short_combos.iter().sorted_by_key(|&(t, _)| (t.len(), t)) {
        // println!("{:6} {t}", combos.len());
        // if t == "wggbugrgwrugguggrwwgbwgrrrrgwgbbwbugrbwgbrgr" {
        //     println!("{:6} {t}", combos.len());
        //     for v in combos {
        //         println!("{}", v.join(","));
        //     }
        // }

        //next, create ???

        //need to sort and dedup again
    }
    0
    // this seems to not be the way
}
fn add_combos_abandon2<'a>(
    hm: &mut HashMap<&'a str, Vec<Vec<&'a str>>>,
    av: &[&'a str],
    s: &'a str,
    _step: usize,
) {
    if hm.contains_key(s) {
        return;
    }
    let mut cs = HashSet::new();
    if av.contains(&s) {
        cs.insert(vec![s]);
    }
    for i in 1..s.len() {
        let (l, r) = s.split_at(i);
        // if _step < 21 { println!("{:_step$} {_step:3} {i:2} {:2} {:12} {l} {r}", "", s.len(), cs.len()); }
        add_combos_abandon2(hm, av, l, _step + 1);
        add_combos_abandon2(hm, av, r, _step + 1);
        // println!("{:_step$} {_step:3} {i:2} {:2} {:8} {:8} {:16} {:12} {l} {r}", "", s.len(), hm[l].len(), hm[r].len(), hm[l].len() * hm[r].len(), cs.len());
        for lc in &hm[l] {
            if lc.is_empty() {
                continue;
            }
            for rc in &hm[r] {
                if rc.is_empty() {
                    continue;
                }
                let mut lrcs = lc.clone();
                lrcs.extend(rc);
                lrcs.sort();
                cs.insert(lrcs);
            }
        }
    }
    hm.insert(s, cs.drain().collect());
}
fn count_towels_combos_abandon2(d: &str) -> usize {
    let (av, de) = parse_avail_desired(d);
    let mut hm = HashMap::new();
    for &s in &av {
        add_combos_abandon2(&mut hm, &av, s, 0);
        println!("{s} {}", hm[s].len());
    }
    for &s in &de {
        add_combos_abandon2(&mut hm, &av, s, 0);
        println!("{s} {}", hm[s].len());
    }
    // for x in hm {
    //     println!("{} {:?}", x.0, x.1);
    // }

    de.iter().map(|&s| hm[s].len()).sum()
}
fn get_starts_with_padded(av: &[&str], t: &str) -> Vec<String> {
    av.iter()
        .copied()
        .filter(|f| t.starts_with(f))
        .map(|f| {
            let mut s = " ".to_owned();
            s.push_str(&f.split("").skip(1).take(f.len()).join("_"));
            s
        })
        .collect()
}
fn append_space(v: &mut [String]) {
    v.iter_mut().for_each(|s| s.push(' '))
}
fn get_starts_with(av: &[&str], t: &str) -> Vec<String> {
    av.iter()
        .copied()
        .filter(|f| t.starts_with(f))
        .map(&str::to_owned)
        .sorted()
        .collect()
}
#[derive(Debug)]
struct Combos {
    top: RefCell<Weak<Self>>,
    parent: RefCell<Weak<Self>>,
    suffs: RefCell<BTreeMap<String, Rc<Self>>>,
    existing: RefCell<BTreeMap<BTreeMap<String, usize>, usize>>,
    count: RefCell<usize>,
}
impl Combos {
    fn empty() -> Rc<Self> {
        Rc::new(Self {
            top: RefCell::new(Weak::new()),
            parent: RefCell::new(Weak::new()),
            suffs: RefCell::new(BTreeMap::new()),
            existing: RefCell::new(BTreeMap::new()),
            count: RefCell::new(0),
        })
    }
    fn get_empty_top() -> Rc<Self> {
        let c = Self::empty();
        *c.top.borrow_mut() = Rc::downgrade(&c);
        c
    }
    fn get_empty_child(self: &Rc<Self>) -> Rc<Self> {
        let c = Self::empty();
        *c.top.borrow_mut() = self.top.borrow().clone();
        *c.parent.borrow_mut() = Rc::downgrade(self);
        c
    }
    fn print_level(self: &Rc<Self>, i: usize) {
        for suff in self.suffs.borrow().iter() {
            println!("{:-<i$}{}", "", suff.0);
            suff.1.print_level(i + suff.0.len());
        }
    }
    fn print(self: &Rc<Self>) {
        self.print_level(0);
    }
    fn get_flattened_strs(self: &Rc<Self>) -> Vec<String> {
        let mut v = Vec::new();
        for suff in self.suffs.borrow().iter() {
            let pre = suff.0.split("").skip(1).take(suff.0.len()).join("_");
            if suff.1.suffs.borrow().is_empty() {
                v.push(pre);
            } else {
                for suff2 in suff.1.get_flattened_strs() {
                    v.push(format!("{pre} {suff2}"));
                }
            }
        }
        v
    }
    fn get_flattened_vecs(self: &Rc<Self>) -> Vec<Vec<String>> {
        self.get_flattened_strs()
            .into_iter()
            .map(|s| s.split(" ").map(|s| s.split("_").join("")).collect())
            .collect()
    }
    fn get_flattened_vecs_sorted_by_frags(self: &Rc<Self>) -> Vec<Vec<String>> {
        let mut v = self.get_flattened_vecs();
        for v in v.iter_mut() {
            v.sort();
        }
        v.sort();
        v
    }
    fn get_flattened_strs_sorted_by_frags(self: &Rc<Self>) -> Vec<String> {
        let mut v = self.get_flattened_vecs_sorted_by_frags();
        v.into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|s| s.split("").skip(1).take(s.len()).join("_"))
                    .join(" ")
            })
            .collect()
    }
    fn get_combo_vec(self: &Rc<Self>) -> Vec<String> {
        let mut v = Vec::new();
        if let Some(p) = self.parent.borrow().upgrade() {
            v.extend(p.get_combo_vec());
            for suff in p.suffs.borrow().iter() {
                if std::rc::Rc::ptr_eq(self, suff.1) {
                    v.push(suff.0.to_owned());
                }
            }
        }
        v
    }
    fn get_combo_btreemap(self: &Rc<Self>) -> BTreeMap<String, usize> {
        self.get_combo_vec()
            .into_iter()
            .counts()
            .into_iter()
            .collect()
    }
    fn is_same_combo_as_other1(self: &Rc<Self>) -> bool {
        let mut self_combo = self.get_combo_vec();
        self_combo.sort();
        let mut other_combos = self.top.borrow().upgrade().unwrap().get_flattened_vecs();
        for ot in other_combos.iter_mut() {
            ot.sort();
        }
        let filtered = other_combos
            .into_iter()
            .filter(|ot| ot == &self_combo)
            .collect_vec();
        if filtered.len() > 2 {
            panic!("dedup failed somewhere {filtered:?}");
        }
        filtered.len() == 2
    }
    fn build_step1(self: &Rc<Self>, av: &[&str], t: &str) {
        let av = &av.iter().copied().filter(|f| t.contains(f)).collect_vec();
        for f in get_starts_with(av, t) {
            let l = f.len();
            let nt = &t[l..];
            let end = t == f;
            let c = self.get_empty_child();
            let c_clone = Rc::clone(&c);
            let f_clone = f.clone();
            self.suffs.borrow_mut().insert(f, c);
            c_clone.build_step1(av, nt);
            if (c_clone.suffs.borrow().is_empty() && !end) || c_clone.is_same_combo_as_other1() {
                self.suffs.borrow_mut().remove(&f_clone);
            }
        }
    }
    fn combo_exists(self: &Rc<Self>, p: &[String]) -> bool {
        if p.is_empty() {
            panic!("no empty");
        }
        let key = &p[0];
        let mut res = self.suffs.borrow().contains_key(key);
        if res && p.len() > 1 {
            let next = &self.suffs.borrow()[key];
            res = res && next.combo_exists(&p[1..]);
        }
        res
    }
    fn is_same_combo_as_other2(self: &Rc<Self>) -> bool {
        let mut self_combo = self.get_combo_vec();
        if self_combo.is_empty() {
            return false;
        }
        self_combo.sort();
        let top = self.top.borrow().upgrade().unwrap();
        top.combo_exists(&self_combo)
    }
    fn is_same_combo_as_other3(self: &Rc<Self>, self_combo: &BTreeMap<String, usize>) -> bool {
        if self_combo.is_empty() {
            return false;
        }
        let top = self.top.borrow().upgrade().unwrap();
        let existing = top.existing.borrow();
        if existing.contains_key(self_combo) {
            existing[self_combo] == 2
        } else {
            false
        }
    }
    fn add_combo(self: &Rc<Self>, self_combo: &BTreeMap<String, usize>) {
        if self_combo.is_empty() {
            return;
        }
        let top = self.top.borrow().upgrade().unwrap();
        let mut existing = top.existing.borrow_mut();
        let entry = existing.entry(self_combo.clone()).or_insert(0);
        *entry += 1;
    }
    fn remove_combo(self: &Rc<Self>, self_combo: &BTreeMap<String, usize>) {
        if self_combo.is_empty() {
            return;
        }
        let top = self.top.borrow().upgrade().unwrap();
        let mut existing = top.existing.borrow_mut();
        let entry = existing.entry(self_combo.clone()).or_insert(0);
        *entry -= 1;
    }
    fn build_step2(self: &Rc<Self>, av: &[&str], t: &str) {
        let av = &av.iter().copied().filter(|f| t.contains(f)).collect_vec();
        for f in get_starts_with(av, t) {
            let l = f.len();
            let nt = &t[l..];
            let end = t == f;
            let c = self.get_empty_child();
            let c_clone = Rc::clone(&c);
            let f_clone = f.clone();
            self.suffs.borrow_mut().insert(f, c);
            let self_combo = c_clone.get_combo_btreemap();
            c_clone.add_combo(&self_combo);
            if c_clone.is_same_combo_as_other3(&self_combo) {
                c_clone.remove_combo(&self_combo);
                self.suffs.borrow_mut().remove(&f_clone);
            } else {
                c_clone.build_step2(av, nt);
                if c_clone.suffs.borrow().is_empty() && !end {
                    c_clone.remove_combo(&self_combo);
                    self.suffs.borrow_mut().remove(&f_clone);
                } else {
                    if end {
                        *self.top.borrow().upgrade().unwrap().count.borrow_mut() += 1;
                    }
                    // println!("{}", c_clone.get_combo_vec().into_iter().map(|s| s.split("").skip(1).take(s.len()).join("_")).join(" "));
                }
            }
        }
    }
    fn build(av: &[&str], t: &str, k: usize, p: bool) -> Rc<Self> {
        let c = Self::get_empty_top();
        if k == 1 {
            if p {
                println!("build_step1");
            }
            c.build_step1(av, t);
        } else {
            if p {
                println!("build_step2");
            }
            c.build_step2(av, t);
        }
        c
    }
    fn get_count_with_step(self: &Rc<Self>, fs: &[&str], av_filtered: &[&str]) -> usize {
        let x = *self.top.borrow().upgrade().unwrap().count.borrow();
        // assert_eq!(fs.len(), x);
        x
    }
    fn get_count_with(self: &Rc<Self>, av: &[&str]) -> usize {
        // let fs = self.get_flattened_strs();
        // let fs = &fs.iter().map(String::as_str).collect_vec();
        // let t = fs[0].split([' ', '_']).join("");
        // let mut av_filtered = av.iter().filter(|&s| s.len() != 1 && t.contains(s)).copied().collect_vec();
        // av_filtered.sort();
        // // println!("{} {:?}", av_filtered.len(), av_filtered);
        // self.get_count_with_step(fs, &av_filtered)

        self.get_count_with_step(&[], &[])
    }
}
/*
b b b b b w b b g g g w b r w w r g b g r b w w u_g g u_b b r b b g b u_b g g g g b u_b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w u_g g u_b b r b b g b u_b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w u_g g u_b b r b b g b_u b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w u_g g_u b b r b b g b u_b g g g g b u_b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w u_g g_u b b r b b g b u_b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w u_g g_u b b r b b g b_u b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g u_b b r b b g b u_b g g g g b u_b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g u_b b r b b g b u_b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g u_b b r b b g b_u b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g_u b b r b b g b u_b g g g g b u_b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g_u b b r b b g b u_b g g g g b_u b r w r g r b b
b b b b b w b b g g g w b r w w r g b g r b w w_u g g_u b b r b b g b_u b g g g g b_u b r w r g r b b
b b b b b b b b b b b b b b b b b b b_u b_u g g g g g g g g g g g g g_u r r r r r r r w w w w w w w_u
b b b b b b b b b b b b b b b b b b b_u b_u g g g g g g g g g g g g_u r r r r r r r u_g w w w w w w w
b b b b b b b b b b b b b b b b b b b_u g g g g g g g g g g g g g_u r r r r r r r u_b w w w w w w w_u
b b b b b b b b b b b b b b b b b b b_u g g g g g g g g g g g g_u r r r r r r r u_b u_g w w w w w w w
b b b b b b b b b b b b b b b b b b g g g g g g g g g g g g g_u r r r r r r r u_b u_b w w w w w w w_u
b b b b b b b b b b b b b b b b b b g g g g g g g g g g g g_u r r r r r r r u_b u_b u_g w w w w w w w
b b b b b b b b b b b b b b b b b b_u b_u g g g g g g g g g g g g g r r r r r r r u_b w w w w w w w_u
b b b b b b b b b b b b b b b b b b_u b_u g g g g g g g g g g g g r r r r r r r u_b u_g w w w w w w w
b b b b b b b b b b b b b b b b b b_u g g g g g g g g g g g g g r r r r r r r u_b u_b w w w w w w w_u
b b b b b b b b b b b b b b b b b b_u g g g g g g g g g g g g r r r r r r r u_b u_b u_g w w w w w w w
b b b b b b b b b b b b b b b b b g g g g g g g g g g g g g r r r r r r r u_b u_b u_b w w w w w w w_u
b b b b b b b b b b b b b b b b b g g g g g g g g g g g g r r r r r r r u_b u_b u_b u_g w w w w w w w
64 ["bb", "bbb", "bbg", "bbgb", "bbr", "bbw", "bg", "bgb", "bgg", "bgr", "br", "brb", "brw", "bu", "bub", "bw", "bwb", "bww", "gb", "gbg", "gbu", "gg", "ggb", "ggg", "ggu", "ggub", "ggwb", "ggwbr", "gr", "grb", "grbww", "gu", "gub", "gw", "gwb", "rb", "rbb", "rbbgb", "rbw", "rg", "rgb", "rgr", "rgrb", "rw", "rwr", "rww", "ub", "ubb", "ubg", "ubr", "ug", "ugg", "uggu", "wbb", "wbr", "wr", "wrg", "wrgr", "wu", "wug", "wugg", "ww", "wwr", "wwu"]
12



*/
trait JoinCharsOfString<T> {
    fn join_chars_of_string(&self, sep: &str) -> String;
}
impl JoinCharsOfString<&str> for &str {
    fn join_chars_of_string(&self, sep: &str) -> String {
        self.split("").skip(1).take(self.len()).join(sep)
    }
}
fn count_towel_combos(av_short: &[&str], av_full: &[&str], t: &str) {
    let c = Combos::build(av_short, t, 2, false);
    // c.print();
    // for s in c.get_flattened_strs() {
    //     println!("{s}");
    // }
    // for s in c.get_flattened_strs_sorted_by_frags() {
    //     println!("{s}");
    // }

    let base = c.get_count_with(av_full);
    println!("{:3} shorts", base);
    let sorted_filt_full = av_full
        .iter()
        .filter(|&v| !av_short.contains(v) && t.contains(v))
        .copied()
        .sorted_by_cached_key(|&v| {
            let s = Instant::now();
            let av_short = av_short
                .iter()
                .copied()
                .chain(std::iter::once(v))
                .collect::<Vec<_>>();
            let c = Combos::build(&av_short, t, 2, false);
            let count = c.get_count_with(av_full);
            println!("{:3} {:?} {v}", count, s.elapsed());
            count
        })
        .collect_vec();

    for i in 1..sorted_filt_full.len() {
        let s = Instant::now();
        let sff = sorted_filt_full[0..i].iter().copied().collect_vec();
        let av_short = av_short
            .iter()
            .copied()
            .chain(sff.clone())
            .collect::<Vec<_>>();
        let c = Combos::build(&av_short, t, 2, false);
        println!(
            "{:7} {:?} {:2} {sff:?}",
            c.get_count_with(av_full),
            s.elapsed(),
            sff.len()
        );
    }

    // let mut t = av
    //     .iter()
    //     .copied()
    //     .filter(|f| t.contains(f))
    //     .map(|f| {
    //         let mut s = " ".to_owned();
    //         s.push_str(&f.split("").skip(1).take(f.len()).join("_"));
    //         s
    //     })
    //     .collect_vec();
    // t.sort();
    // for t in t {
    //     print!("{t} ");
    // }
    // println!(" ");
}
fn count_towels_combos(d: &str) -> usize {
    let s = Instant::now();
    let (mut av_short, de) = parse_avail_desired(d);
    let av_full = av_short.clone();
    dedup_av(&mut av_short);
    // av_short.push("uggu");
    // av_short.push("ggub");
    // av_short.sort();

    // let hm = get_lookup(&av_short);
    // let mut ts = check_towels_possible(&hm, &de).remove(&true).unwrap();
    // ts.sort();
    // println!("{ts:?}");

    // count_towel_combos(&["a", "b", "ab"], "abab");

    // count_towel_combos(&av_short, &av_full, "rubrub");
    // count_towel_combos(&av_short, &av_full, "ubur");
    // count_towel_combos(&av_short, &av_full, "uburgub");

    let x = &[
        "wwb", "bwwrw", "wbr", "ub", "uubwuwg", "gwgg", "uuw", "rbr", "bgurbgub", "gbubwru",
        "uugwww", "rgr", "rw", "gr", "gw", "rur", "buuw", "gwugw", "bgbb", "wgb", "uwu", "ubb",
        "ggrbr", "wuuwg", "wub", "gurr", "bugg", "ruruub", "guw", "bguugrg", "uwgr", "wggb", "bw",
        "uuu", "ggwb", "uwbwur", "grb", "r", "ruwu", "ubw", "rgg", "bwguw", "ggu", "ruw", "gb",
        "grr", "uruwr", "wuwww", "gww", "wrgwuu", "bgrwru", "rr", "bwr", "rgb", "bwb", "bur",
        "bbrw", "rrwb", "gwr", "w", "wgrrw", "rwub", "gguruuuw", "uur", "rrrbbrwb", "urb", "ubwr",
        "uggwgw", "buwgu", "wrbwru", "wru", "wwuwubub", "bugbbbub", "bwubbr", "gbu", "ggwr",
        "ggbugu", "uuug", "wgbw", "buurbw", "rwww", "gwbg", "wurw", "rrrru", "wwggbgr", "uubr",
        "bgrr", "bwgb", "uuuug", "wg", "wwwu", "wbgbgwb", "guu", "gbbwu", "uurgw", "gwbuu",
        "uwrbug", "rrwbrugg", "ubbb", "uwr", "rrbbbw", "wbuw", "wugwrb", "rguru", "ugub", "buw",
        "wwruwb", "wgbrb", "rgw", "bgb", "rrur", "b", "bub", "rubuguw", "brgwg", "br", "wrw",
        "brbrwg", "uwrg", "guwb", "guubwu", "uurg", "wgugrwbg", "gg", "wgubwggu", "urw", "wbuu",
        "gggr", "uwgbwrrr", "guwwg", "uuwb", "bwww", "gub", "rwgbg", "uwugr", "gbrb", "urr", "rwr",
        "bruww", "uw", "wbwuguu", "ugg", "rug", "wwggu", "rwbb", "ur", "wwr", "bgw", "bbwrgw",
        "rgbwug", "bggug", "bbru", "wgugb", "rbw", "rbburw", "rwrwr", "bbgb", "rwb", "wu", "bgg",
        "gbrwwrur", "wgwg", "rbbgb", "ubuwb", "gubbb", "rbu", "urubggr", "wbgbg", "brr", "wwu",
        "wgug", "gwu", "uru", "urgb", "uggbugw", "bggrr", "rg", "wbrrb", "bwrw", "ubg", "uub",
        "gwubwuu", "rrrrr", "guru", "bg", "wgw", "wguug", "bruuuu", "rub", "ggbgr", "grgrwgw",
        "gggurbgw", "brwrgg", "bb", "rrww", "uruwrrug", "brbr", "grggwr", "rgwbw", "bbrubb",
        "brrw", "bbuurwb", "rwggbww", "uwurggg", "wur", "brrwru", "rrgwr", "gurrb", "brgggw",
        "wbgwb", "ggr", "gwbww", "wbg", "rwg", "wguu", "wuggr", "gur", "ubr", "bwg", "gu", "rgrb",
        "wrgr", "wwbuwww", "gbg", "wrr", "rburgu", "ug", "rubu", "bbw", "rggwb", "gbw", "wrg",
        "gggwbur", "brg", "wbubbu", "bu", "uuwr", "rgbwrbu", "gwgr", "rwgbgg", "gbrr", "wuu",
        "bbgr", "gbgrwgr", "wggbbw", "bgr", "bwubg", "uurbbgr", "uug", "ru", "wgu", "uuruggg",
        "ubrubu", "wgrr", "rgwubb", "rgwuu", "wgg", "gbbgwr", "uwb", "rubgbrgr", "brgu", "rbbgru",
        "uwuwwg", "ggb", "bwwru", "gbgu", "wuw", "buubug", "brruww", "ururbwru", "ubbrwu", "bru",
        "rbrbr", "rrr", "wuub", "rwu", "gubwrgg", "ubrbbug", "urg", "rguug", "wbrwb", "wrbrg",
        "rbg", "urbuwr", "wrbww", "brw", "gru", "rrb", "urgbbw", "wugw", "ggub", "wwg", "bgbwb",
        "wbgg", "bubgr", "wug", "ggwbuuu", "rrg", "rrw", "wuugbrg", "bubuw", "ugwgwr", "brurwb",
        "rggu", "uwrr", "rbrru", "wrb", "rrbbwr", "rgru", "wrgw", "wwuw", "bug", "buruw", "buug",
        "bbb", "bbwr", "ugu", "bbu", "gwgrr", "ugbubg", "ggg", "gwgwbu", "grg", "bwu", "wugg",
        "wrbu", "ubu", "wgr", "gwrrur", "brb", "uuggub", "wuwbu", "wwrbr", "ubgu", "ggwbr",
        "rurugw", "ubrbr", "rww", "gwbgwwww", "rrbb", "ugbg", "grwwb", "gbr", "ww", "uwbbugb",
        "ubgw", "urwr", "uuwugg", "wwgu", "wbrgwg", "uwgu", "rgwrwurr", "wrwbwwr", "ugr", "urbgg",
        "wuwu", "rwguww", "wrubg", "wurg", "wwbwb", "rwuwurwg", "g", "wr", "rbb", "rgu", "bwug",
        "ugw", "uwbgw", "rgrwu", "uwruub", "brbrb", "gurwr", "wwguubw", "gwg", "wruur", "rgwr",
        "urbuubr", "ruwguru", "gwubr", "gwgw", "uuguu", "brgruw", "burrrru", "bbwu", "rggg",
        "wbrb", "brug", "uww", "guur", "grw", "bbgub", "wrwr", "rru", "wwbbr", "gubwrbrg", "gug",
        "ugb", "bbrwru", "bgwuwb", "wbgb", "gwrgbr", "wgggrbg", "grbgg", "rrbbu", "uwbr", "bbg",
        "wggbwr", "rgrbbgr", "bwrrww", "wgbb", "wbb", "uruwbwr", "gwbbwwr", "wbw", "ubbrbgw",
        "wubrugrr", "rrggu", "bgu", "urbw", "grbu", "gugru", "bbrrbb", "wubrrgbr", "gwb", "bww",
        "gwuu", "gwuuurw", "rwuubbr", "gbgbr", "urwubww", "rrburb", "buwr", "gbb", "uggu", "ruug",
        "rgrrugu", "guug", "ubgrwu", "grgwgrg", "grbww", "wbu", "rwuu", "gbur", "wrwgw", "bbr",
        "rb", "uwggb",
    ];
    // count_towel_combos(x, "bbbw"); // short but good to test dedup
    // count_towel_combos(x, "uburgubbuugrub"); // good to test complex dedup
    // count_towel_combos(x, "bbbwbbgggwbrw"); // 2 s; good to test time

    count_towel_combos(
        &av_short,
        &av_full,
        "bbbbbwbbgggwbrwwrgbgrbwwuggubbrbbgbubggggbubrwrgrbb",
    );

    // ts.into_iter()
    //     .sorted_by_key(|v| v.len())
    //     .inspect(|t| println!("{t}"))
    //     .map(|t| count_towel_combos(&av_short, &av_full, t))
    //     .for_each(drop);

    println!("{:?}", s.elapsed());
    0
}

fn count_towel_basic<'a>(t: &'a str, av: &[&'a str], hm: &Rc<RefCell<HashMap<&'a str, usize>>>) -> usize {
    if t.is_empty() {
        1
    } else {
        let mut res = 0;
        for a in av.iter().copied() {
            if let Some(t) = t.strip_prefix(a) {
                if ! hm.borrow().contains_key(&t) {
                    let inter = count_towel_basic(t, av, hm);
                    hm.borrow_mut().insert(t, inter);
                };
                res += hm.borrow().get(&t).unwrap();
            }
        }
        res
    }
}
fn count_towels_basic(d: &str) -> usize {
    let (av, de) = parse_avail_desired(d);
    let mut sum = 0;
    for t in de.iter() {
    // for t in std::iter::once("bbbbbwbbgggwbrwwrgbgrbwwuggubbrbbgbubggggbubrwrgrbb") {
        let s = Instant::now();
        let hm: Rc<RefCell<HashMap<&str, usize>>> = Rc::new(RefCell::new(HashMap::new()));
        sum += count_towel_basic(t, &av, &hm);
        println!("{:20?} {t}", s.elapsed());
    }
    sum
}
fn main() {
    println!("START");
    println!("{}", count_towels_basic(d()));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1_e1_get_avail_desired() {
        let (av, _) = parse_avail_desired(e1());
        assert_eq!(av, vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"]);
    }
    #[test]
    fn t2_e1_get_avail_desired() {
        let (_, de) = parse_avail_desired(e1());
        assert_eq!(
            de,
            vec![
                "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"
            ]
        );
    }
    #[test]
    fn t1_e1_count_towels_that_are_possible1() {
        assert_eq!(count_towels_that_are_possible_from_data1(e1()), 6);
    }
    #[test]
    fn t1_e1_count_towels_that_are_possible2() {
        assert_eq!(count_towels_that_are_possible_from_data2(e1()), 6);
    }
    #[test]
    fn t1_d_count_towels_that_are_possible() {
        assert_eq!(count_towels_that_are_possible_from_data2(d()), 344);
    }
    #[test]
    fn t1_e1_count_towels_basic() {
        assert_eq!(count_towels_basic(e1()), 16);
    }
    #[test]
    fn t1_d_count_towels_basic() {
        assert_eq!(count_towels_basic(d()), 996172272010026);
    }
}
