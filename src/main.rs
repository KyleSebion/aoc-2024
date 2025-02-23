#![allow(dead_code)]
use std::{iter::once, time::Instant};
use itertools::{self, Itertools};

fn s1() -> &'static str {
    "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"
}
fn s2() -> &'static str {
    "\
26,35
61,49
14,27
66,63
11,16
55,19
44,15
61,69
19,34
24,37
5,10
1,19
50,11
64,61
58,21
64,41
1,14
59,68
29,43
19,35
19,11
49,67
51,13
65,47
11,38
11,46
3,4
17,14
69,37
69,66
31,37
64,39
57,65
13,25
53,58
59,52
4,25
59,32
65,28
5,47
67,46
58,15
61,25
4,13
33,43
56,59
63,19
11,31
17,41
12,33
57,41
5,26
63,26
62,43
13,4
1,31
69,55
5,44
5,51
23,47
53,15
15,15
13,2
13,9
66,33
61,57
5,23
49,69
29,13
51,57
51,37
21,32
59,20
9,45
4,33
59,35
63,25
15,27
63,33
51,30
60,61
55,31
59,59
67,57
27,21
69,63
63,55
23,49
4,7
65,23
7,21
55,50
59,48
10,21
49,19
57,67
49,32
5,29
61,59
5,45
1,43
55,55
59,70
9,7
61,53
10,9
7,13
7,48
7,34
60,17
61,63
5,40
49,65
2,15
53,29
65,19
57,52
62,55
15,18
13,51
16,19
55,49
3,31
69,35
55,28
66,49
3,29
1,9
3,42
14,13
57,16
58,43
3,15
5,14
60,59
5,33
21,47
3,51
9,21
66,57
57,20
59,19
40,3
1,46
3,5
55,35
4,19
9,49
59,60
12,49
12,25
51,35
3,12
7,39
23,11
51,16
55,67
17,8
67,50
8,11
55,32
63,66
1,3
13,11
11,10
55,33
63,34
63,62
46,49
61,15
57,48
27,13
8,27
51,42
60,53
57,31
58,61
7,37
53,43
69,59
20,27
4,23
27,46
29,12
62,33
24,11
13,29
67,47
48,21
11,11
15,6
14,11
5,7
10,53
55,68
67,61
5,9
57,32
2,41
58,45
24,43
2,39
23,7
63,30
15,43
1,37
67,36
3,37
15,3
4,21
19,7
51,33
49,48
7,41
15,25
8,17
57,17
2,9
51,55
19,37
63,47
12,13
5,8
11,29
57,37
7,2
54,55
4,17
55,45
25,9
64,57
69,65
67,48
58,65
16,43
52,69
6,23
8,3
63,56
59,14
5,27
47,66
9,1
11,51
11,4
17,43
3,30
51,67
64,51
61,48
11,27
61,35
8,5
2,5
3,47
61,31
5,35
70,35
13,18
53,69
23,13
4,27
63,61
5,39
19,9
31,39
15,31
22,45
17,49
14,21
65,55
9,51
49,23
57,18
19,28
70,65
3,19
9,22
10,19
62,47
69,68
7,25
59,17
11,47
14,23
13,15
1,11
5,0
51,29
49,13
1,47
8,49
20,45
17,50
57,40
63,45
31,43
13,35
63,23
7,5
63,31
3,43
56,43
11,5
56,41
21,11
64,47
53,31
1,34
1,6
62,15
19,36
7,51
11,19
13,33
23,41
27,39
15,38
7,8
19,38
17,11
63,63
9,46
47,29
11,41
24,45
1,29
11,23
1,4
68,55
25,42
10,23
51,20
67,59
53,64
7,20
59,34
13,43
6,21
53,67
17,28
62,51
65,53
64,45
45,67
59,38
7,33
65,63
0,51
64,27
7,3
60,27
60,57
50,69
7,44
52,33
9,29
19,29
52,25
53,19
2,11
1,21
7,7
14,35
11,6
59,45
7,9
15,16
19,24
21,31
21,8
57,47
13,19
7,31
29,23
47,27
67,40
57,38
55,22
70,33
3,49
5,36
65,64
14,51
12,45
53,22
59,27
47,55
21,46
18,49
11,50
21,15
9,3
30,41
57,69
64,23
69,69
4,3
9,18
51,22
65,35
62,65
65,51
23,39
6,47
0,21
23,29
62,19
1,48
67,51
69,53
9,52
5,30
2,17
21,36
17,25
3,53
50,15
65,43
9,17
4,45
53,33
1,44
63,68
62,63
61,24
15,7
61,42
6,11
19,43
9,14
18,39
67,37
61,67
11,48
24,41
55,41
65,59
63,37
35,45
21,40
8,19
27,40
9,26
5,52
67,39
51,34
5,19
6,33
59,51
2,23
63,14
62,39
61,41
19,13
67,56
53,41
17,44
11,53
14,41
63,57
15,32
59,33
1,27
61,39
61,19
17,17
67,58
15,21
66,61
69,50
69,61
9,53
70,47
2,53
3,35
53,65
4,35
53,52
56,65
1,23
45,28
3,1
15,37
0,39
62,59
55,17
53,37
53,21
11,33
24,33
25,13
59,50
13,8
9,23
51,41
66,41
25,47
9,15
3,11
25,45
15,39
63,43
15,9
5,21
20,43
18,41
42,19
24,49
15,19
9,5
59,15
69,60
11,26
16,13
60,67
63,22
9,24
64,33
55,26
11,2
17,16
19,47
67,41
59,30
22,19
29,48
61,27
4,49
52,37
61,36
57,29
1,53
15,24
61,22
45,58
65,67
57,21
56,25
57,19
63,67
63,27
55,23
63,44
3,36
59,21
1,49
5,13
56,21
27,45
15,30
13,28
6,5
57,68
24,47
29,17
65,39
5,5
15,17
1,8
30,37
64,37
61,18
56,61
1,28
1,1
27,41
61,33
53,42
63,29
6,39
57,43
13,49
63,49
11,7
65,57
69,67
32,9
1,13
13,39
66,37
24,53
13,34
55,13
48,33
53,57
57,49
3,13
59,63
21,44
5,28
66,43
54,63
51,39
7,19
6,19
3,2
61,40
47,15
57,51
19,16
50,67
53,59
53,35
13,42
54,37
51,61
25,34
2,31
3,16
49,17
8,53
61,61
7,17
1,7
69,54
55,65
1,41
69,51
21,12
66,35
1,33
1,5
56,39
42,69
53,66
18,47
48,67
15,33
3,45
54,11
67,69
6,3
5,25
3,27
8,29
21,9
59,24
21,33
60,13
65,65
19,20
5,41
54,39
26,47
68,65
57,23
0,13
51,19
1,45
52,19
65,37
10,35
61,34
13,7
19,31
7,27
13,13
5,37
53,25
51,31
3,38
25,39
61,68
57,63
61,51
13,17
67,35
13,30
4,41
12,11
7,14
59,43
8,1
68,41
1,51
1,26
6,51
20,33
7,36
51,45
9,34
65,49
11,28
13,40
67,33
33,53
55,43
11,17
65,70
55,62
68,43
22,9
5,15
64,19
65,45
5,43
63,69
63,52
53,40
21,39
63,58
9,31
59,13
68,51
69,41
1,39
53,30
2,45
59,29
7,1
11,3
7,45
5,6
1,42
13,20
7,16
67,65
59,36
51,23
1,35
2,19
58,35
69,46
15,22
69,44
32,11
54,67
54,25
19,45
24,17
3,21
1,36
59,47
2,27
23,40
61,29
56,33
57,39
57,35
57,66
67,64
7,38
16,29
56,35
18,13
23,38
23,36
11,1
65,42
26,39
34,33
61,37
49,70
16,9
54,35
23,14
50,63
67,63
21,49
11,25
57,36
60,41
63,35
19,10
9,30
3,33
59,22
16,11
60,65
14,9
7,42
59,67
59,57
13,52
50,39
55,15
61,65
55,16
65,69
69,39
6,27
59,64
23,37
21,37
33,39
19,41
3,39
59,37
51,18
7,29
54,43
48,29
15,41
21,38
54,19
61,50
26,49
19,15
60,11
9,6
53,23
4,53
25,17
7,12
55,69
17,20
69,33
11,14
65,68
26,13
5,1
23,33
1,52
5,17
7,32
63,32
59,41
55,14
57,30
11,12
57,44
69,49
61,45
55,27
32,47
9,50
54,31
59,23
67,68
7,46
56,13
59,31
19,39
13,44
13,32
13,31
65,24
69,57
11,21
69,62
3,41
57,45
11,32
10,29
53,27
66,53
61,28
15,13
7,24
50,27
15,49
13,27
51,14
11,0
59,54
27,44
59,65
15,11
67,52
61,17
58,27
0,1
65,33
16,41
63,39
55,25
59,69
63,48
20,13
54,23
59,53
13,41
56,17
61,13
68,37
61,23
21,48
61,43
13,37
5,3
64,59
60,45
57,12
10,15
61,47
61,46
21,45
15,26
53,39
19,25
13,21
2,21
51,69
17,45
65,27
7,43
57,57
9,42
9,25
22,25
69,38
17,13
5,54
9,32
67,45
9,27
12,23
70,57
59,25
53,68
15,48
52,27
62,25
55,21
7,30
2,49
66,67
16,47
69,43
12,17
47,14
17,37
15,50
59,39
67,55
63,41
20,19
49,30
14,43
57,33
63,51
66,45
61,55
17,39
8,37
57,27
13,23
4,11
24,7
18,45
3,25
7,35
57,62
3,7
67,43
68,59
69,47
5,31
10,3
1,54
67,67
67,53
9,11
21,22
3,9
15,36
51,32
23,10
21,34
9,9
55,29
11,9
52,15
47,69
9,33
12,37
54,59
61,21
68,33
63,54
55,37
8,9
65,54
21,35
12,41
57,24
55,39
17,34
13,53
17,19
10,39
20,11
45,48
45,26
53,28
64,65
51,40
3,50
18,9
65,38
67,49
1,32
9,13
59,26
61,30
69,45
61,38
63,53
21,41
11,36
4,47
49,21
51,38
11,35
3,32
57,58
65,61
59,28
23,30
63,59
21,42
59,11
55,48
49,63
16,39
55,56
39,15
8,43
3,23
12,7
68,61
7,15
10,43
45,1
32,35
33,30
4,57
48,13
38,57
45,3
14,47
47,43
7,65
33,23
19,59
24,63
55,57
39,56
18,31
29,66
12,53
34,13
69,5
35,39
19,33
29,37
61,16
51,3
5,61
7,69
9,55
25,31
50,5
40,15
23,20
64,1
45,41
57,11
23,21
26,59
62,11
37,19
49,4
30,63
36,61
41,38
56,29
31,59
31,19
31,57
67,21
45,13
65,15
34,35
34,29
33,9
33,54
3,69
59,10
23,58
27,69
33,27
42,53
5,65
67,1
65,31
39,30
22,5
40,19
5,67
29,53
17,55
49,62
44,19
37,61
21,3
59,7
65,25
27,55
39,17
38,61
44,47
63,9
15,1
26,65
57,53
23,63
33,21
39,70
39,14
31,60
19,63
21,29
1,60
35,37
19,3
15,69
47,17
65,5
9,65
33,58
15,55
22,53
39,27
6,67
46,27
27,25
47,39
23,17
27,5
23,56
32,41
36,37
2,57
33,35
18,53
59,2
43,4
67,7
21,53
44,7
27,3
13,59
27,43
26,43
31,21
33,1
55,10
31,70
9,67
43,34
19,49
35,61
19,23
35,11
47,52
21,67
28,63
65,8
3,65
27,18
27,53
68,31
37,5
35,29
37,28
33,11
35,0
35,40
46,35
27,17
51,21
43,21
36,57
42,41
41,17
22,13
21,17
49,27
33,47
39,33
19,18
26,51
35,65
21,19
16,67
41,55
20,51
33,15
5,64
1,25
27,7
27,56
43,33
49,11
40,21
7,62
27,29
35,67
63,12
23,15
25,53
43,45
11,39
13,45
5,62
1,57
13,5
37,34
47,65
43,13
27,51
39,21
33,69
39,26
31,38
17,15
3,60
25,23
65,6
30,19
51,53
37,59
41,21
30,25
21,63
27,67
34,25
2,67
29,26
39,6
23,4
38,65
52,61
29,2
37,42
31,25
22,17
33,0
41,9
5,55
23,22
28,15
43,28
9,43
46,61
41,51
43,17
34,65
33,49
5,53
17,1
31,65
15,67
47,2
43,37
46,13
15,47
39,31
61,7
17,67
1,61
53,55
17,61
57,3
47,61
53,49
47,6
41,40
16,53
25,43
1,15
37,39
15,56
67,20
48,7
64,17
19,60
43,36
39,62
22,33
17,59
8,67
49,45
50,37
32,51
33,59
15,23
43,32
13,70
33,29
22,59
51,17
37,23
21,64
10,59
45,44
49,25
33,63
29,11
67,25
29,40
53,0
39,46
25,49
39,25
43,1
68,7
31,63
51,49
63,15
7,67
49,54
17,47
37,37
65,2
22,27
23,1
28,5
20,53
39,66
25,69
65,16
41,19
32,27
69,19
28,51
33,61
29,31
35,1
17,32
44,37
39,69
26,9
39,50
58,1
43,29
12,67
41,3
36,63
59,55
32,49
37,64
68,25
33,26
51,1
15,68
66,13
7,49
13,1
43,0
37,47
35,54
33,20
47,35
43,35
23,19
19,65
51,15
45,49
36,33
49,35
38,7
42,37
57,13
43,57
33,5
41,26
27,59
34,39
35,5
67,9
19,64
25,14
55,6
27,11
29,46
10,47
45,63
36,11
43,49
7,11
61,3
50,53
26,37
31,16
33,44
48,23
41,62
35,19
54,45
37,49
39,29
25,52
67,22
17,21
43,44
29,54
29,34
40,45
17,58
13,65
30,55
26,69
53,9
43,54
43,15
36,27
19,26
35,53
30,43
25,12
51,25
43,9
44,17
43,43
63,8
43,41
20,7
45,5
51,24
15,29
64,29
31,6
33,16
43,23
47,67
55,46
16,55
53,1
31,27
43,59
18,55
43,47
34,51
43,25
47,22
42,51
55,47
29,57
11,67
47,45
17,53
35,47
18,51
44,31
12,59
25,28
61,2
43,66
38,17
58,57
31,15
25,35
26,61
31,14
18,69
46,37
37,7
25,7
21,25
19,5
19,55
41,56
37,31
37,27
57,15
39,13
45,25
41,69
34,47
27,37
41,1
55,63
9,66
43,12
13,60
59,1
64,11
37,36
41,22
49,61
63,17
37,45
13,57
31,67
53,17
41,13
27,10
52,3
41,27
35,63
57,59
41,10
3,56
28,37
69,31
62,5
21,65
41,49
35,33
26,25
47,21
69,9
39,48
44,39
31,31
9,64
24,23
64,5
30,45
39,57
53,50
39,5
8,63
34,59
15,63
27,22
66,3
5,11
45,11
30,23
55,9
29,65
55,61
23,61
33,40
24,57
31,41
23,5
29,49
69,15
49,15
44,43
69,26
14,55
42,27
36,3
27,68
23,45
12,65
3,63
23,55
53,61
35,38
41,29
26,31
23,69
37,17
39,8
43,10
23,53
46,41
45,51
32,3
46,9
35,49
45,24
51,9
20,57
49,39
57,61
19,61
23,57
17,35
23,66
31,23
23,23
41,60
13,69
37,18
42,59
45,27
35,3
67,5
34,21
46,57
41,57
45,17
41,59
36,55
48,17
27,32
39,0
38,41
27,15
9,69
29,61
38,3
11,37
9,39
9,19
41,34
13,63
52,45
39,61
17,3
4,67
27,16
19,68
45,21
37,3
54,13
23,2
53,53
47,24
30,13
17,69
35,36
37,33
3,61
45,8
37,25
24,69
27,34
29,39
57,9
50,35
28,49
28,23
43,61
49,33
17,5
15,59
36,41
7,59
25,1
32,7
19,19
51,56
39,18
29,33
33,7
69,1
44,29
66,19
33,37
45,39
69,17
25,15
25,37
49,37
42,17
57,10
27,65
41,11
41,14
37,65
33,55
24,59
16,57
37,9
37,52
43,39
33,19
37,35
29,7
31,5
27,23
7,47
33,13
41,47
17,63
27,27
41,7
49,64
14,15
67,28
70,23
9,59
5,57
57,7
47,47
28,29
31,64
39,68
4,61
67,18
52,35
68,17
36,49
9,35
31,53
25,6
38,25
66,31
39,37
25,25
31,22
7,23
47,3
41,41
25,41
67,15
59,56
41,39
65,7
29,25
48,35
11,13
59,5
31,49
40,39
67,29
42,25
70,5
47,41
33,31
29,52
32,45
3,66
31,55
39,55
45,55
35,25
49,49
41,35
48,41
37,1
32,19
36,67
39,51
34,61
55,52
46,47
39,1
57,46
69,28
41,24
22,63
49,55
31,13
14,5
45,9
49,51
29,69
39,44
54,1
19,66
49,41
63,5
41,16
23,25
19,53
47,19
14,69
49,43
2,63
43,55
23,68
31,7
39,45
11,62
30,47
11,15
29,15
45,22
26,1
21,13
19,30
15,60
44,57
15,5
43,68
44,55
41,63
53,51
25,67
43,14
43,51
25,62
41,67
66,7
37,2
35,27
19,67
67,30
45,2
31,47
60,5
39,63
37,51
47,37
70,19
39,59
37,15
36,13
9,37
33,56
47,1
31,11
17,24
3,67
35,32
34,27
27,19
37,68
31,50
8,61
29,9
55,53
32,33
30,5
35,13
43,50
11,63
37,57
53,6
18,5
35,24
37,46
45,20
37,43
23,67
12,63
60,9
40,51
45,12
49,2
15,65
67,11
69,27
37,53
59,61
49,58
54,9
48,59
55,51
21,54
41,53
11,69
21,23
41,37
29,63
37,11
11,59
26,15
3,59
39,58
19,51
21,24
21,7
49,29
3,55
39,11
65,9
21,55
39,24
8,59
33,41
50,47
33,62
27,28
44,9
37,21
17,22
19,69
25,3
50,21
33,64
39,23
38,35
47,25
63,7
43,6
28,25
63,10
50,45
27,35
43,63
20,5
13,3
41,45
46,19
33,33
61,9
57,54
45,15
31,68
30,57
15,46
52,47
44,67
53,13
37,6
41,15
57,4
49,42
9,56
66,25
27,49
43,5
23,60
27,54
42,45
42,47
55,54
1,67
45,57
45,61
45,29
19,58
53,48
65,29
35,69
47,9
37,48
23,27
52,11
32,17
53,60
63,13
11,45
63,3
47,57
57,8
37,30
47,33
32,57
66,11
11,61
43,67
28,1
17,33
35,51
19,27
45,42
11,57
68,9
23,3
55,5
14,61
32,67
23,65
49,50
47,49
29,68
69,13
3,58
27,63
59,3
7,55
1,59
21,69
33,57
25,51
34,19
11,65
65,41
29,3
30,9
39,47
3,57
6,55
11,58
37,69
36,59
20,15
48,61
47,59
13,67
1,65
24,51
45,23
31,28
17,9
33,51
69,4
27,57
53,3
39,53
23,16
31,4
54,17
47,16
48,19
38,51
25,20
20,61
41,65
11,68
17,27
32,53
35,52
49,0
37,63
40,53
63,21
59,49
39,43
65,21
51,11
9,68
39,54
49,5
20,3
5,69
39,10
29,47
13,61
11,54
43,20
41,25
63,1
41,43
0,59
33,17
29,35
65,17
25,19
68,1
45,19
17,62
17,2
10,61
15,64
46,51
15,2
24,25
17,7
36,5
35,35
41,5
43,11
18,67
34,69
39,39
67,19
63,4
11,55
29,41
31,45
41,8
39,19
17,23
47,44
41,30
50,9
35,41
47,32
41,36
41,31
57,55
27,33
44,61
65,1
1,68
19,57
25,54
43,7
56,1
26,19
31,61
61,11
21,27
49,31
51,7
45,7
55,1
47,11
53,63
51,50
29,51
45,43
45,64
19,1
29,36
11,56
29,67
28,43
35,43
49,59
40,63
1,69
25,61
19,22
16,5
51,62
25,57
39,12
48,39
46,53
43,27
31,3
21,30
27,66
37,13
59,9
37,20
23,35
51,54
3,64
29,45
13,55
44,3
22,49
51,2
21,57
52,55
25,33
69,21
69,25
30,59
38,39
51,5
65,3
39,41
57,1
1,17
61,1
45,45
27,1
46,63
27,47
45,47
1,55
45,59
35,23
17,31
29,62
52,7
37,22
47,53
47,7
51,47
35,4
65,20
35,57
25,26
23,9
37,67
57,25
29,56
7,58
35,31
27,4
32,23
46,5
24,1
15,57
11,66
34,49
28,39
49,53
66,15
41,42
19,21
62,3
45,69
31,51
40,59
31,33
47,46
55,3
37,14
41,2
69,23
35,44
31,69
39,42
67,23
69,3
21,5
9,61
15,0
48,65
15,35
37,16
22,65
27,61
67,6
47,68
7,57
25,22
47,51
21,56
45,34
40,65
6,59
67,31
33,67
69,29
25,63
45,66
25,29
28,31
6,57
57,5
47,31
31,29
30,29
31,17
35,55
35,56
47,13
14,65
35,59
41,68
20,67
9,47
33,45
25,5
51,43
36,31
39,32
51,44
63,11
32,31
48,55
16,3
29,55
1,62
28,11
42,57
67,13
43,53
37,29
5,63
47,63
49,1
15,62
28,61
56,7
13,58
45,35
19,6
19,17
34,11
17,65
33,2
29,5
69,7
40,5
31,66
35,9
49,47
44,63
5,49
51,27
46,17
3,3
43,31
15,61
51,51
16,33
30,31
25,55
51,63
9,63
49,26
69,30
49,8
17,57
41,23
23,31
25,27
43,3
11,49
39,36
36,45
5,59
56,3
43,69
35,18
25,8
54,7
45,40
47,56
51,12
49,7
35,8
67,27
45,60
29,27
23,28
15,53
15,51
45,31
22,69
7,53
42,11
45,54
9,57
37,24
16,65
48,45
53,5
30,33
66,1
27,31
29,59
8,55
43,19
21,21
49,57
23,43
51,58
55,59
69,16
1,63
58,7
65,13
25,59
53,47
53,7
32,61
8,69
6,65
45,53
34,5
39,7
48,11
51,65
69,12
45,33
4,69
62,7
26,57
51,59
28,59
68,3
29,1
39,28
68,21
23,59
1,66
28,7
7,63
21,62
44,51
31,24
49,28
39,3
31,1
25,11
39,9
25,65
29,19
31,9
68,13
42,63
70,11
50,59
33,42
46,69
32,37
55,11
3,17
54,3
41,33
51,52
40,33
65,11
67,10
11,43
33,14
29,29
45,65
21,50
55,7
21,43
33,3
30,1
29,20
13,47
37,10
59,4
39,49
5,70
34,7
35,15
47,38
28,19
30,17
29,70
23,51
25,30
53,45
29,10
53,4
33,25
17,51
60,7
35,21
26,5
47,10
47,5
46,31
25,66
29,8
18,63
67,17
15,45
43,65
51,8
67,3
35,16
7,61
31,35
42,31
47,23
35,22
1,24
36,9
41,61
39,65
17,60
57,50
9,41
21,59
33,65
47,4
43,22
17,26
22,1
17,36
35,66
21,61
39,35
35,7
20,1
21,51
9,40
27,9
25,21
18,3
36,69
25,2
39,67
53,11
63,65
29,21
41,48
61,5
49,3
69,11
37,41
35,17
65,14
67,24
15,52
52,65
37,55
21,1
49,9
17,29
42,3
45,37
48,51
22,30
64,58
20,23
46,14
42,18
18,54
38,13
38,33
7,64
68,2
47,26
70,40
55,0
18,23
38,36
6,54
12,55
60,28
38,1
0,49
47,62
15,40
70,42
68,50
28,62
23,6
61,8
62,40
36,16
10,56
24,32
50,55
6,37
61,60
0,41
63,42
7,40
14,19
41,20
48,38
40,18
7,52
54,28
40,30
38,37
2,52
7,60
32,25
44,8
41,4
4,68
68,30
26,67
28,26
12,69
46,38
18,25
56,15
24,34
38,45
49,66
34,37
58,8
0,35
47,40
38,10
44,13
57,0
12,8
15,34
50,40
40,28
10,0
20,62
56,31
3,18
34,0
54,0
32,40
16,37
28,68
52,50
36,51
19,12
54,53
10,55
0,63
18,70
32,36
46,48
36,29
0,12
2,46
6,36
2,24
48,56
26,42
2,20
36,53
51,26
12,12
32,20
4,26
36,68
20,24
22,3
46,44
18,30
56,14
7,50
0,58
56,4
20,21
22,67
18,16
16,56
15,12
10,37
18,37
17,66
28,70
5,68
42,5
8,36
62,14
12,4
8,33
45,52
36,48
50,38
14,3
30,21
26,46
37,12
52,26
50,26
25,32
6,70
31,10
12,57
42,9
14,56
4,55
46,54
44,56
0,60
64,34
30,36
49,14
48,14
42,68
68,0
29,30
28,48
22,32
19,54
8,70
68,60
21,18
38,29
31,54
2,66
36,8
48,22
65,36
42,6
47,30
20,44
0,37
0,40
40,47
68,28
34,23
42,0
6,38
8,51
8,26
44,40
33,28
16,70
1,12
66,23
35,6
28,2
16,68
15,42
2,55
47,28
54,69
38,52
67,4
38,14
38,11
58,18
38,44
46,24
28,33
2,37
35,12
30,14
58,39
16,23
44,60
19,42
24,40
33,12
14,18
64,42
23,24
50,44
50,14
6,16
28,46
46,11
62,41
46,46
2,51
10,62
44,10
18,29
18,24
59,18
2,33
0,16
20,38
14,42
42,52
6,66
43,26
56,30
48,20
33,46
0,17
20,20
40,57
20,64
40,10
2,65
43,2
46,23
26,7
48,60
50,12
14,58
63,50
10,63
68,47
34,10
34,4
38,53
38,48
17,0
57,42
36,50
66,24
70,39
29,22
41,58
6,61
16,10
38,23
36,35
9,36
38,38
23,32
31,2
40,23
40,50
54,27
17,6
62,0
42,56
14,66
70,66
25,10
53,46
9,70
10,57
18,21
3,52
34,28
22,0
19,56
65,4
44,16
24,10
24,30
12,68
44,42
34,14
20,16
32,1
23,52
38,32
3,62
42,61
24,14
46,59
40,29
18,62
64,28
63,40
14,4
16,58
22,64
63,2
60,1
66,39
14,40
22,8
20,55
50,13
12,56
6,1
7,68
4,52
39,38
2,64
20,17
40,34
5,12
18,42
36,26
32,18
27,64
6,56
50,48
62,38
0,57
70,51
70,43
65,40
4,40
46,40
48,63
42,38
20,49
64,36
52,52
33,4
68,6
6,26
16,25
30,54
10,69
34,9
18,33
0,61
2,54
68,26
56,57
36,52
60,31
42,20
25,38
3,0
24,38
60,14
22,23
28,6
0,46
44,50
61,70
18,43
4,14
37,4
44,66
62,8
70,50
50,54
66,0
8,2
52,54
3,46
32,22
54,56
44,18
36,47
47,50
38,55
6,15
13,6
12,9
70,4
27,48
20,26
7,26
70,6
66,5
32,24
6,63
33,6
70,44
26,24
5,42
30,68
40,41
22,38
2,60
5,34
1,70
14,60
54,41
11,22
66,64
67,14
50,60
30,20
33,22
6,64
59,0
53,10
32,44
51,48
32,6
30,48
23,42
2,4
45,46
24,42
2,18
40,2
54,58
27,60
26,26
13,12
32,68
0,44
39,4
69,42
47,36
58,19
12,6
24,12
41,46
48,69
18,34
48,52
67,2
20,32
61,4
67,12
17,12
22,29
48,53
66,28
21,66
54,34
46,30
20,14
24,66
52,39
37,70
46,56
32,55
46,4
69,8
22,68
42,33
18,61
48,26
18,20
48,25
34,31
1,20
40,35
20,9
52,5
15,70
62,32
40,7
21,70
24,48
3,68
4,1
4,39
43,48
61,66
59,46
49,24
34,1
12,42
29,28
32,30
24,44
41,32
59,66
2,22
14,0
4,8
0,28
0,52
14,29
9,0
58,62
50,19
20,39
48,3
23,70
22,15
20,65
26,4
56,2
70,49
54,68
43,60
34,36
0,11
54,42
34,12
48,42
44,26
58,12
48,2
26,34
22,31
53,54
26,33
10,25
64,20
49,40
58,2
22,47
48,70
8,28
38,54
52,6
10,17
34,6
28,69
52,32
16,8
50,66
8,42
54,36
10,64
25,60
59,40
11,18
60,49
62,54
14,67
3,28
44,35
5,18
69,14
49,34
14,33
23,62
10,67
45,0
53,2
39,64
31,52
35,20
42,64
13,66
32,12
4,43
3,10
18,46
54,15
2,3
70,22
66,54
66,59
17,18
0,48
59,42
48,58
16,62
68,19
55,2
67,62
29,0
54,14
65,10
16,52
58,20
28,67
69,34
56,27
30,3
4,46
0,2
11,8
12,44
40,66
18,66
2,29
6,50
33,68
27,36
54,60
70,24
22,61
33,52
28,42
60,64
34,38
67,70
6,31
39,20
42,48
64,44
6,30
44,28
2,7
22,34
70,7
57,64
56,68
36,28
60,33
22,54
37,56
4,24
45,6
10,48
31,12
55,12
35,14
45,18
34,52
68,24
12,21
16,59
8,15
20,52
14,22
24,55
68,57
12,70
43,24
60,56
38,21
70,27
66,47
0,69
51,70
31,0
62,64
70,14
52,18
66,58
48,9
34,64
61,58
18,60
35,10
18,68
52,34
64,15
62,42
3,54
44,25
35,58
46,22
62,44
25,46
14,64
14,1
68,23
50,64
43,62
26,22
10,46
35,64
12,54
51,0
0,25
63,60
58,13
46,55
59,8
5,24
10,27
22,28
10,34
32,63
52,20
58,55
58,17
8,34
60,42
54,20
24,16
26,66
37,62
52,63
6,41
20,47
1,10
56,56
52,22
58,14
30,12
44,24
33,34
44,58
34,45
24,60
56,54
37,40
36,23
51,68
43,30
35,30
4,12
31,32
68,64
34,50
38,26
14,32
55,30
70,63
60,12
16,21
48,5
34,18
10,70
20,60
64,6
32,5
3,20
54,21
24,15
8,62
70,41
47,64
32,32
66,16
20,12
42,44
24,52
8,60
16,69
16,27
62,69
18,6
48,28
48,31
67,44
49,18
58,34
18,17
69,56
30,39
19,46
45,62
7,56
68,4
68,22
12,34
20,59
53,18
40,24
39,52
26,52
10,10
40,6
36,56
40,37
58,54
54,26
58,23
6,62
45,38
26,32
0,22
16,4
68,16
48,47
40,46
35,70
62,23
47,48
8,24
34,26
68,10
4,34
23,54
28,0
0,65
70,31
34,8
45,36
22,20
58,52
62,67
44,64
4,64
62,52
46,64
38,68
9,16
16,17
4,62
16,64
12,24
60,66
44,0
29,18
9,10
17,56
40,54
64,55
17,46
36,1
44,22
58,44
34,56
4,65
7,0
42,10
49,68
24,70
38,43
42,12
44,11
8,68
21,26
54,18
64,12
48,49
17,68
52,57
58,48
55,58
6,46
31,62
5,32
40,31
16,14
30,40
50,46
32,14
36,38
28,58
48,4
14,14
30,7
36,17
36,25
9,54
3,48
52,14
10,60
46,18
52,44
5,48
26,6
53,32
27,50
58,32
30,32
25,56
50,25
54,40
32,59
14,36
2,2
24,0
38,69
50,7
50,57
22,18
11,20
69,24
63,20
60,39
68,36
8,0
55,60
43,8
39,2
46,66
58,11
6,45
70,16
17,70
28,8
4,51
25,4
40,69
43,18
16,30
32,54
70,13
58,31
20,42
31,40
8,16
43,56
60,15
67,60
48,30
14,57
25,70
27,8
20,35
38,24
70,37
18,1
16,36
58,51
42,67
4,4
32,64
68,14
70,68
68,38
20,48
56,26
68,40
31,58
37,8
5,66
39,34
34,17
61,10
44,5
38,46
52,49
53,8
31,42
2,61
56,9
32,69
13,36
52,68
64,25
68,32
36,44
68,62
0,27
56,6
29,44
58,70
24,9
8,54
4,63
61,6
41,0
16,18
46,62
46,50
48,32
5,22
6,20
34,42
28,47
40,64
5,4
48,44
58,26
30,51
42,62
42,2
12,38
53,56
3,6
56,0
7,10
14,53
70,36
28,60
18,38
16,35
17,54
43,52
4,6
0,33
1,2
10,26
62,37
64,24
51,60
22,56
66,69
32,60
38,47
46,32
26,28
38,19
0,9
49,46
47,18
12,18
15,10
20,31
32,28
55,4
26,53
12,43
30,16
44,32
70,28
42,49
44,4
"
}
fn e1() -> (usize, usize, usize, &'static str) {
    (7, 7, 12, s1())
}
fn e1_map() -> &'static str {
    "\
...#...
..#..#.
....#..
...#..#
..#..#.
.#..#..
#.#....
"
}
fn d1() -> (usize, usize, usize, &'static str) {
    (71, 71, 1024, s2())
}

fn map_to_str(m: &[Vec<char>]) -> String {
    m.iter()
        .flat_map(|r| r.iter().chain(once(&'\n')).copied())
        .collect()
}
fn get_map((w, h, l, b): (usize, usize, usize, &str)) -> Vec<Vec<char>> {
    let mut m = (0..h).map(|_| vec!['.'; w]).collect::<Vec<_>>();
    for (x, y) in b.lines().take(l).map(|p| {
        p.split(',')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }) {
        m[y][x] = '#';
    }
    m
}
const DIRS: [(usize, usize); 4] = [
    (1, 0),
    (0, 1),
    (usize::MAX, 0),
    (0, usize::MAX),
];
fn is_backwards(&(dx1, dy1): &(usize, usize), &(dx2, dy2): &(usize, usize)) -> bool {
    if (dx1 == 0 && dy1 == 0) || (dx2 == 0 && dy2 == 0) {
        false
    } else {
        (dx1 == dx2 && dy1 != dy2) || (dy1 == dy2 && dx1 != dx2)
    }
}
fn walk_paths(mut m: Vec<Vec<char>>, (x, y): (usize, usize), &(dx, dy): &(usize, usize), s: usize, ss: &mut Vec<Vec<usize>>) -> Vec<Vec<Vec<char>>> {
    let mut v = Vec::new();
    let nx = x.wrapping_add(dx);
    let ny = y.wrapping_add(dy);
    if nx < m.first().unwrap().len() && ny < m.len() && m[ny][nx] != '#' && m[ny][nx] != 'O' && s < ss[ny][nx] {
        m[ny][nx] = 'O';
        ss[ny][nx] = s;
        // if s % 200 == 0 {
        //     println!("{}", map_to_str(&m));
        // }
        if nx == m.first().unwrap().len() - 1 && ny == m.len() - 1 {
            v.push(m);
        } else {
            for d in DIRS.iter().filter(|&dxdy| !is_backwards(dxdy, &(dx, dy))) {
                v.extend(walk_paths(m.clone(), (nx, ny), d, s + 1, ss));
            }
        }
    }
    v
}
fn get_paths(m: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut ss = (0..(m.len())).map(|_| vec![usize::MAX; m.first().unwrap().len()]).collect::<Vec<_>>();
    walk_paths(m.to_vec(), (0, 0), &(0, 0), 0, &mut ss)
}
fn get_min_steps(ms: &[Vec<Vec<char>>]) -> usize {
    if let Some(c) = ms.iter().map(|m| m.iter().flatten().counts()[&'O']).min() {
        c - 1
    } else {
        0
    }
}
fn get_min_map(ms: &[Vec<Vec<char>>]) -> Option<String> {
    let min = get_min_steps(ms) + 1;
    if let Some(m) = ms.iter().find(|m| m.iter().flatten().counts()[&'O'] == min) {
        return Some(map_to_str(m));
    }
    None
}
fn find_when_blocked(d: (usize, usize, usize, &str)) -> &str {
    let mut d = d;
    let m = d.3.lines().count() + 1;
    for c in (0..m).rev() {
        // let st = Instant::now();
        // print!("{c} ");
        d.2 = c;
        let m = &get_map(d);         // does take(c); take(2) would yield nth(0) and nth(1)
        let ms = &get_paths(m);
        let ss = get_min_steps(ms);
        // println!("{s} {:?}", st.elapsed());
        // if let Some(str) = get_min_map(ms) {
        //     print!("{str}");
        // }
        if ss != 0 {
            return d.3.lines().nth(c).unwrap();       // if c == 2 like in comment above, nth(2) would yield the line after nth(0) and nth(1) (the bad line)
        }
    }
    unreachable!()
}

fn main() {
    let s = Instant::now();
    println!("{} {:?}", find_when_blocked(d1()), s.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test] fn is_backwards_0_0_ok() { assert_eq!(DIRS.to_vec(), DIRS.iter().copied().filter(|dxdy| !is_backwards(dxdy, &(0, 0))).collect::<Vec<_>>()) }
    #[test] fn e1_map_ok() { assert_eq!(e1_map(), map_to_str(&get_map(e1()))) }
    #[test] fn e1_steps_ok() { assert_eq!(22, get_min_steps(&get_paths(&get_map(e1())))) }
    #[test] fn d1_steps_ok() { assert_eq!(404, get_min_steps(&get_paths(&get_map(d1())))) }
    #[test] fn d1_blocker() { assert_eq!("27,60", find_when_blocked(d1())) }
}
