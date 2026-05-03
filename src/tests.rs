use cryptominisat::Solver;
use crate::sudoku;

pub fn test() {
    let mut solver = Solver::new();

    let sudoku = [
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
        [
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
            [solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var(), solver.new_var()],
        ],
    ];

    sudoku::setup(&mut solver, &sudoku);

    let tasks = [
        "094000130000000000000076002080010000032000000000200060000050400000008007006304008",
        "000000000000942080160000029000000008906000001400250000004000000020008090050000700",
        "000007000090001000000045006000020000036000410500000809000000004000018000081500032",
        "052470000060000000000008010400000009700950000020040030000800090000003706000091000",
        "090000000001006000060080070300000010000039000000050002170400028000003000086000057",
        "000005000020004010030080020000008400800600000090010705006000000950003060003000001",
        "500068000000000060042050000000800900001000040903000620700001009004200003080000000",
        "070021004000030000601000002000000060008600703190000040010000208420900000000000000",
        "000000001007050309004800020000000000030005700009420000000003000001000407060278000",
        "000008003016020907030004600000000000905000200020130009003000020070005000000000400",
        "004020030000809000000000700050037008000000005049060010500000000068000000007040901",
        "000006003009040005320000008000010000001750609200000080000060000000800040470000200",
        "000008020000006930098070001000000000009210000700000096240090000000300180000000003",
        "002046000004080005070030009000002000305700000700000400006000093000054078000000000",
        "003000040400200000000090026000070000010902000260000008500007000000060803300000069",
        "000000003005002014000080060000000000946000000030004206000700000000030680070291000",
        "020000000000004800054018030700001004000086050000000600000000100000020009230400005",
        "009043000000000030410070000000000000800500060040006002000000010004098006700600520",
        "000000000406070090050038200000000030900000000004260000070003002001600800085000700",
        "000604000000000003010002600002000000600090015804000006000007000976050000000203100",
    ];

    let solutions = [
        "794582136268931745315476982689715324432869571157243869821657493943128657576394218",
        "249186573735942186168375429512697348976834251483251967694723815327518694851469732",
        "653287941794631258128945376819724563236859417547163829965372184372418695481596732",
        "152479683368215974974638512416387259783952461529146837237864195891523746645791328",
        "894317265731526894562984173358642719247139586619758432173465928925873641486291357",
        "168295374529734618437186529312578496875649132694312785786951243951423867243867951",
        "597468132318927564642153897456832971821796345973514628735641289164289753289375416",
        "379521684284736519651498372732845961548619723196273845917364258425987136863152497",
        "526394871817652349394817526148736952632985714759421683975143268281569437463278195",
        "742698513816523947539714682381952764965847231427136859653479128174285396298361475",
        "184726539376859142925413786652137498713984625849265317591372864468591273237648951",
        "715986423689243715324175968963418572841752639257639184132564897596827341478391256",
        "156938427427156938398472561534689712869217354712543896243891675675324189981765243",
        "582946731934187625671235849168492357345768912729513486456871293293654178817329564",
        "623815947479236185851794326934678251718952634265143798596387412142569873387421569",
        "762149853385672914194583762527316498946827531831954276453768129219435687678291345",
        "128397546397654812654218937786531294942786351513942678469875123875123469231469785",
        "569143287287965431413872695635729148872514963941386752326457819154298376798631524",
        "893426175426571398157938246512789634968314527734265981679853412241697853385142769",
        "289634751465719823713582649152376984637498215894125376321947568976851432548263197",
    ];

    for i in 0..tasks.len() {
        let solution = sudoku::solve(&sudoku, &mut solver, tasks[i].trim()).unwrap();
        println!("Sudoku:   {}", tasks[i]);
        println!("Computed: {}", solution);
        println!("Solution: {}\n", solutions[i]);
        assert!(solution == solutions[i]);
    }
}
